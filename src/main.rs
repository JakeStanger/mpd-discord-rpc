use std::time::Duration;

use discord_rpc_client::Client as DiscordClient;
use mpd_client::responses::{PlayState, Song};
use mpd_client::{commands, Client as MPDClient};
use regex::Regex;

use crate::album_art::AlbumArtClient;
use crate::mpd_conn::get_timestamp;
use config::Config;

mod album_art;
mod config;
mod mpd_conn;

pub const IDLE_TIME: u64 = 5;
pub const ACTIVE_TIME: u64 = 1;

/// Attempts to find a playing MPD host every 5
/// seconds until one is found
async fn idle(hosts: &[String]) -> MPDClient {
    println!("Entering idle mode");

    loop {
        let conn_wrapper = mpd_conn::try_get_mpd_conn(hosts).await;

        if let Some(client) = conn_wrapper {
            println!("Exiting idle mode");
            return client;
        }

        tokio::time::sleep(Duration::from_secs(IDLE_TIME)).await;
    }
}

#[tokio::main]
async fn main() {
    let re = Regex::new(r"\$(\w+)").unwrap();

    let config = Config::load();
    let format = &config.format;

    let details_tokens = get_tokens(&re, &format.details);
    let state_tokens = get_tokens(&re, &format.state);
    let large_text_tokens = get_tokens(&re, &format.large_text);
    let small_text_tokens = get_tokens(&re, &format.small_text);

    // MPD and Discord connections
    let mut mpd = idle(&config.hosts).await;
    let mut drpc = DiscordClient::new(config.id);

    let mut album_art_client = AlbumArtClient::new();

    drpc.start();

    // Main program loop - keep updating state until exit
    loop {
        let state = mpd_conn::get_status(&mpd).await.state;

        if state == PlayState::Playing {
            let current_song = mpd.command(commands::CurrentSong).await;
            if let Ok(Some(song_in_queue)) = current_song {
                let song = song_in_queue.song;

                let details =
                    replace_tokens(&format.details, &details_tokens, &song, &mut mpd).await;
                let state = replace_tokens(&format.state, &state_tokens, &song, &mut mpd).await;
                let large_text =
                    replace_tokens(&format.large_text, &large_text_tokens, &song, &mut mpd).await;
                let small_text =
                    replace_tokens(&format.small_text, &small_text_tokens, &song, &mut mpd).await;

                let timestamps = get_timestamp(&mut mpd, format.timestamp).await;

                let url = album_art_client.get_album_art_url(song).await;

                let res = drpc.set_activity(|act| {
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
                                assets = assets.large_text(large_text)
                            }
                            if !small_text.is_empty() {
                                assets = assets.small_text(small_text)
                            }
                            assets
                        })
                        .timestamps(|_| timestamps)
                });

                if let Err(why) = res {
                    eprintln!("Failed to set activity: {:?}", why);
                };
            }
        } else {
            if let Err(why) = drpc.clear_activity() {
                eprintln!("Failed to clear activity: {}", why);
            };

            mpd = idle(&config.hosts).await;
        }

        // sleep for 1 sec to not hammer the mpd and rpc servers
        tokio::time::sleep(Duration::from_secs(ACTIVE_TIME)).await;
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
async fn replace_tokens(
    format_string: &str,
    tokens: &Vec<String>,
    song: &Song,
    mpd: &mut MPDClient,
) -> String {
    let mut compiled_string = format_string.to_string();
    for token in tokens {
        let value = mpd_conn::get_token_value(mpd, song, token).await;
        compiled_string = compiled_string.replace(format!("${}", token).as_str(), value.as_str());
    }
    compiled_string
}
