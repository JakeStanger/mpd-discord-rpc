mod config;
mod defaults;
mod mpd_conn;

use config::Config;
use defaults::{ACTIVE_TIME, DETAILS_FORMAT, IDLE_TIME, STATE_FORMAT};
use discord_rpc_client::Client as DiscordClient;
use mpd::{Client as MPDClient, Song, State};
use regex::{Captures, Regex};
use std::{thread, time};
use crate::mpd_conn::get_timestamp;
use crate::defaults::TIMESTAMP_MODE;

/// Attempts to find a playing MPD host every 5
/// seconds until one is found
fn idle(hosts: &[String]) -> MPDClient {
    println!("Entering idle mode");

    loop {
        let conn_wrapper = mpd_conn::try_get_mpd_conn(hosts);

        if conn_wrapper.is_some() {
            println!("Exiting idle mode");
            return conn_wrapper.unwrap();
        }

        thread::sleep(time::Duration::from_secs(IDLE_TIME));
    }
}

fn main() {
    let re = Regex::new(r"\$(\w+)").unwrap();

    // Load config and defaults if necessary
    let config = Config::load();

    let hosts = &config.hosts;

    let format_options = config.format;

    let details_format = match format_options.as_ref() {
        Some(options) => options.details.as_str(),
        None => DETAILS_FORMAT,
    };

    let state_format = match format_options.as_ref() {
        Some(options) => options.state.as_str(),
        None => STATE_FORMAT,
    };

    let timestamp_mode = match format_options.as_ref() {
        Some(options) => match options.timestamp.as_ref() {
            Some(t) => t.as_str(),
            None => TIMESTAMP_MODE
        },
        None => TIMESTAMP_MODE
    };

    // MPD and Discord connections
    let mut mpd = idle(hosts);
    let mut drpc = DiscordClient::new(config.id);

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

            // set the activity
            if let Err(why) = drpc.set_activity(|act| {
                act.state(state)
                    .details(details)
                    .assets(|asset| asset.small_image("notes"))
                    .timestamps(|timestamps| get_timestamp(&mut mpd, timestamps, timestamp_mode))
            }) {
                println!("Failed to set activity: {}", why);
            };
        } else {
            if let Err(why) = drpc.clear_activity() {
                println!("Failed to clear activity: {}", why);
            };

            mpd = idle(hosts);
        }

        // sleep for 1 sec to not hammer the mpd and rpc servers
        thread::sleep(time::Duration::from_secs(ACTIVE_TIME));
    }
}
