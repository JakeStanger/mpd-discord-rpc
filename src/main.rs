use std::{thread, time};

use discord_rpc_client::Client as DiscordClient;
use mpd::{Client as MPDClient, State};
use regex::{Captures, Regex};

use crate::album_art::AlbumArtClient;
use crate::mpd_conn::get_timestamp;
use config::Config;
use defaults::{ACTIVE_TIME, IDLE_TIME};

mod album_art;
mod config;
mod defaults;
mod mpd_conn;

/// Attempts to find a playing MPD host every 5
/// seconds until one is found
fn idle(hosts: &[String]) -> MPDClient {
    println!("Entering idle mode");

    loop {
        let conn_wrapper = mpd_conn::try_get_mpd_conn(hosts);

        if let Some(client) = conn_wrapper {
            println!("Exiting idle mode");
            return client;
        }

        thread::sleep(time::Duration::from_secs(IDLE_TIME));
    }
}

fn main() {
    let re = Regex::new(r"\$(\w+)").unwrap();

    // Load config and defaults if necessary.
    // We're safe to unwrap everything here since all options should have valid defaults.
    let config = Config::load();
    let id = config.id.unwrap();
    let hosts = &config.hosts.unwrap();
    let format_options = config.format.unwrap();
    let (
        details_format,
        state_format,
        timestamp_mode,
        large_image,
        small_image,
        large_text,
        small_text,
    ) = (
        format_options.details.as_deref().unwrap(),
        format_options.state.as_deref().unwrap(),
        format_options.timestamp.as_deref().unwrap(),
        format_options.large_image.as_deref().unwrap(),
        format_options.small_image.as_deref().unwrap(),
        format_options.large_text.as_deref().unwrap(),
        format_options.small_text.as_deref().unwrap(),
    );

    // MPD and Discord connections
    let mut mpd = idle(hosts);
    let mut drpc = DiscordClient::new(id);

    let mut album_art_client = AlbumArtClient::new();

    drpc.start();

    // Main program loop - keep updating state until exit
    loop {
        let state = mpd_conn::get_status(&mut mpd).state;

        if state == State::Play {
            if let Ok(Some(song)) = mpd.currentsong() {
                let details = re.replace_all(details_format, |caps: &Captures| {
                    mpd_conn::get_token_value(&mut mpd, &song, &caps[1])
                });

                let state = re.replace_all(state_format, |caps: &Captures| {
                    mpd_conn::get_token_value(&mut mpd, &song, &caps[1])
                });

                let large_text = re.replace_all(large_text, |caps: &Captures| {
                    mpd_conn::get_token_value(&mut mpd, &song, &caps[1])
                });

                let small_text = re.replace_all(small_text, |caps: &Captures| {
                    mpd_conn::get_token_value(&mut mpd, &song, &caps[1])
                });

                let res = drpc.set_activity(|act| {
                    act.state(state)
                        .details(details)
                        .assets(|mut assets| {
                            // Attempt to fetch art from MusicBrainz
                            // fall back to configured image
                            let url = album_art_client.get_album_art_url(song);
                            match url {
                                Some(url) => assets = assets.large_image(&url),
                                None => {
                                    if !large_image.is_empty() {
                                        assets = assets.large_image(large_image)
                                    }
                                }
                            };

                            if !small_image.is_empty() {
                                assets = assets.small_image(small_image)
                            }
                            if large_text != "" {
                                assets = assets.large_text(large_text)
                            }
                            if small_text != "" {
                                assets = assets.small_text(small_text)
                            }
                            assets
                        })
                        .timestamps(|timestamps| {
                            get_timestamp(&mut mpd, timestamps, timestamp_mode)
                        })
                });

                if let Err(why) = res {
                    eprintln!("Failed to set activity: {}", why);
                };
            }
        } else {
            if let Err(why) = drpc.clear_activity() {
                eprintln!("Failed to clear activity: {}", why);
            };

            mpd = idle(hosts);
        }

        // sleep for 1 sec to not hammer the mpd and rpc servers
        thread::sleep(time::Duration::from_secs(ACTIVE_TIME));
    }
}
