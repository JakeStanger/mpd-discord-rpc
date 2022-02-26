use std::{thread, time};

use discord_rpc_client::Client as DiscordClient;
use mpd::{Client as MPDClient, Song, State};
use regex::{Captures, Regex};

use config::Config;
use defaults::{
    IDLE_TIME,
    ACTIVE_TIME,
};
use crate::mpd_conn::get_timestamp;

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

    // Load config and defaults if necessary
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

    drpc.start();

    // Main program loop - keep updating state until exit
    loop {
        let state = mpd.status().unwrap().state;

        if state == State::Play {
            let song: Song = mpd.currentsong().unwrap().unwrap();

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

            // set the activity
            if let Err(why) = drpc.set_activity(|act| {
                act.state(state)
                    .details(details)
                    .assets(|mut asset| {
                        if !large_image.is_empty() { asset = asset.large_image(large_image) }
                        if !small_image.is_empty() { asset = asset.small_image(small_image) }
                        if large_text != "" { asset = asset.large_text(large_text) }
                        if small_text != "" { asset = asset.small_text(small_text) }
                        asset
                    })
                    .timestamps(|timestamps| get_timestamp(&mut mpd, timestamps, timestamp_mode))
            }) {
                eprintln!("Failed to set activity: {}", why);
            };
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
