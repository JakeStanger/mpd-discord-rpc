use std::time::Duration;

use discord_presence::models::ActivityType;
use discord_presence::models::EventData;
use discord_presence::{Client as DiscordClient, DiscordError};
use mpd_client::client::ConnectionEvent::SubsystemChange;
use mpd_client::client::Subsystem;
use mpd_client::commands;
use mpd_client::responses::{PlayState, Song, SongInQueue, Status};
use mpd_utils::MultiHostClient;
use regex::Regex;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{debug, error, info};

use crate::album_art::AlbumArtClient;
use crate::mpd_conn::get_timestamp;
use config::Config;

mod album_art;
mod config;
mod mpd_conn;

pub const IDLE_TIME: u64 = 5;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let re = Regex::new(r"\$(\w+)").expect("Failed to parse regex");

    let config = Config::load();
    let format = &config.format;

    let tokens = Tokens {
        details: get_tokens(&re, &format.details),
        state: get_tokens(&re, &format.state),
        large_text: get_tokens(&re, &format.large_text),
        small_text: get_tokens(&re, &format.small_text),
    };

    // MPD and Discord connections
    let mut mpd = MultiHostClient::new(config.hosts.clone(), Duration::from_secs(IDLE_TIME));
    mpd.init();

    let (tx, mut rx) = mpsc::channel(16);
    let mut service = Service::new(&config, tokens, tx);
    service.start();

    loop {
        tokio::select! {
            Ok(event) = mpd.recv() => {
                if matches!(*event, SubsystemChange(Subsystem::Player | Subsystem::Queue)) {
                    info!("Detected change, updating status");
                    debug!("Change: {event:?}");

                    if let Ok((Some(status), current_song)) = mpd.with_client(|client| async move {
                            let status = client.command(commands::Status).await.ok();

                            let current_song = if status.is_some() {
                                client.command(commands::CurrentSong).await.ok().flatten()
                            } else {
                                None
                            };

                            (status, current_song)
                        }).await {
                        service.update_state(&status, current_song).await;
                    }
                }
            }
            Some(event) = rx.recv() => {
                match event {
                    ServiceEvent::Ready => {
                        info!("Connected to Discord");

                        // set initial status as soon as ready
                        if let Ok((Some(status), current_song)) = mpd.with_client(|client| async move {
                            let status = client.command(commands::Status).await.ok();

                            let current_song = if status.is_some() {
                                client.command(commands::CurrentSong).await.ok().flatten()
                            } else {
                                None
                            };

                            (status, current_song)
                        }).await {
                        service.update_state(&status, current_song).await;
                    }
                    },
                    ServiceEvent::Error(err) => {
                        error!("{err}");
                        sleep(Duration::from_secs(IDLE_TIME)).await;
                        service.start();
                    }
                }
            },
        }
    }
}

struct Tokens {
    details: Vec<String>,
    state: Vec<String>,
    large_text: Vec<String>,
    small_text: Vec<String>,
}

enum ServiceEvent {
    Ready,
    Error(String),
}

struct Service<'a> {
    config: &'a Config,
    album_art_client: AlbumArtClient,
    drpc: DiscordClient,
    tokens: Tokens,
}

impl<'a> Service<'a> {
    fn new(config: &'a Config, tokens: Tokens, event_tx: mpsc::Sender<ServiceEvent>) -> Self {
        let event_tx2 = event_tx.clone();

        let drpc = DiscordClient::new(config.id);

        drpc.on_ready(move |_| {
            debug!("Discord RPC ready");
            event_tx
                .try_send(ServiceEvent::Ready)
                .expect("channel to be open");
        })
        .persist();

        drpc.on_error(move |err| {
            if let EventData::Error(err) = err.event {
                let msg = err.message.unwrap_or_default();
                if msg == "Io Err" {
                    event_tx2
                        .try_send(ServiceEvent::Error(msg))
                        .expect("channel to be open");
                }
            }
        })
        .persist();

        let album_art_client = AlbumArtClient::new();
        Self {
            config,
            album_art_client,
            drpc,
            tokens,
        }
    }

    fn start(&mut self) {
        self.drpc.start();
    }

    async fn update_state(&mut self, status: &Status, current_song: Option<SongInQueue>) {
        // https://discord.com/developers/docs/rich-presence/how-to#updating-presence-update-presence-payload
        const MAX_BYTES: usize = 128;

        let format = &self.config.format;

        if matches!(status.state, PlayState::Playing) {
            if let Some(song_in_queue) = current_song {
                let song = song_in_queue.song;

                let details = clamp(
                    replace_tokens(&format.details, &self.tokens.details, &song, status),
                    MAX_BYTES,
                );
                let state = clamp(
                    replace_tokens(&format.state, &self.tokens.state, &song, status),
                    MAX_BYTES,
                );
                let large_text =
                    replace_tokens(&format.large_text, &self.tokens.large_text, &song, status);
                let small_text =
                    replace_tokens(&format.small_text, &self.tokens.small_text, &song, status);

                let timestamps = get_timestamp(status, format.timestamp);

                let url = self.album_art_client.get_album_art_url(song).await;

                let res = self.drpc.set_activity(|act| {
                    act.state(state)
                        ._type(ActivityType::Listening)
                        .details(details)
                        .assets(|mut assets| {
                            match url {
                                Some(url) => assets = assets.large_image(url),
                                None => {
                                    if !format.large_image.is_empty() {
                                        assets = assets.large_image(&format.large_image);
                                    }
                                }
                            };

                            if !format.small_image.is_empty() {
                                assets = assets.small_image(&format.small_image);
                            }
                            if !large_text.is_empty() {
                                assets = assets.large_text(large_text);
                            }
                            if !small_text.is_empty() {
                                assets = assets.small_text(small_text);
                            }
                            assets
                        })
                        .timestamps(|_| timestamps)
                });

                if let Err(why) = res {
                    // api returns a bogus error about missing buttons but succeeds anyway
                    // so don't log it
                    if !matches!(&why, DiscordError::JsonError(err) if err.to_string().starts_with("missing field `buttons`"))
                    {
                        error!("Failed to set activity: {why:?}");
                    }
                };
            }
        } else if let Err(why) = self.drpc.clear_activity() {
            error!("Failed to clear activity: {why:?}");
        }
    }
}

/// Extracts the formatting tokens from a formatting string
fn get_tokens(re: &Regex, format_string: &str) -> Vec<String> {
    re.captures_iter(format_string)
        .map(|caps| caps[1].to_string())
        .collect::<Vec<_>>()
}

/// Replaces each of the formatting tokens in the formatting string
/// with actual data pulled from MPD
fn replace_tokens(
    format_string: &str,
    tokens: &Vec<String>,
    song: &Song,
    status: &Status,
) -> String {
    let mut compiled_string = format_string.to_string();
    for token in tokens {
        let value = mpd_conn::get_token_value(song, status, token);
        compiled_string = compiled_string.replace(format!("${token}").as_str(), value.as_str());
    }
    compiled_string
}

/// Clamps a string to a specified length (byte count).
///
/// If a string is longer than the max length,
/// it is cut down and ellipses are added
/// to make the byte count equal to or just below the max.
fn clamp(mut str: String, len: usize) -> String {
    const ELLIPSES_LEN: usize = 3;

    if str.len() > len {
        while str.len() > (len - ELLIPSES_LEN) {
            str.pop();
        }

        str.push_str("...");
    }

    str
}
