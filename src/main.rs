use std::time::Duration;

use discord_rpc_client::{Client as DiscordClient, Client};
use mpd_client::responses::{PlayState, Song, Status};
use mpd_utils::MultiHostClient;
use regex::Regex;
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
    let hosts = config.hosts.iter().map(String::as_str).collect::<Vec<_>>();
    let mut mpd = MultiHostClient::new(&hosts, Duration::from_secs(IDLE_TIME));
    mpd.init();

    let mut service = Service::new(&config, tokens);

    let status = mpd.status().await;
    match status {
        Ok(status) => service.update_state(&mpd, &status).await,
        Err(err) => eprintln!("{err:?}"),
    }

    while let Some(event) = mpd.recv().await {
        info!("Detected change, updating status");
        debug!("Change: {event:?}");

        let status = mpd.status().await;
        match status {
            Ok(status) => service.update_state(&mpd, &status).await,
            Err(err) => eprintln!("{err:?}"),
        }
    }
}

struct Tokens {
    details: Vec<String>,
    state: Vec<String>,
    large_text: Vec<String>,
    small_text: Vec<String>,
}

struct Service<'a> {
    config: &'a Config,
    album_art_client: AlbumArtClient,
    drpc: Client,
    tokens: Tokens,
}

impl<'a> Service<'a> {
    fn new(config: &'a Config, tokens: Tokens) -> Self {
        let mut drpc = DiscordClient::new(config.id);

        drpc.start();

        let album_art_client = AlbumArtClient::new();
        Self {
            config,
            album_art_client,
            drpc,
            tokens,
        }
    }

    async fn update_state(&mut self, mpd: &MultiHostClient<'a>, status: &Status) {
        let format = &self.config.format;

        if matches!(status.state, PlayState::Playing) {
            let current_song = mpd.current_song().await;
            if let Ok(Some(song_in_queue)) = current_song {
                let song = song_in_queue.song;

                let details = replace_tokens(&format.details, &self.tokens.details, &song, status);
                let state = replace_tokens(&format.state, &self.tokens.state, &song, status);
                let large_text =
                    replace_tokens(&format.large_text, &self.tokens.large_text, &song, status);
                let small_text =
                    replace_tokens(&format.small_text, &self.tokens.small_text, &song, status);

                let timestamps = get_timestamp(status, format.timestamp);

                let url = self.album_art_client.get_album_art_url(song).await;

                let res = self.drpc.set_activity(|act| {
                    act.state(state)
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
                    error!("Failed to set activity: {why:?}");
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
