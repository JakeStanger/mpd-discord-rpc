use std::fs;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::{thread, time};

use dirs::config_dir;
use discord_rpc_client::Client as DiscordClient;
use mpd::{Client as MPDClient, Song, State};
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};

const IDLE_TIME: u64 = 5;
const ACTIVE_TIME: u64 = 1;

const DISCORD_ID: u64 = 677226551607033903;
const DEFAULT_HOST: &str = "localhost:6600";
const DETAILS_FORMAT: &str = "$title";
const STATE_FORMAT: &str = "$artist / $album";

#[derive(Serialize, Deserialize)]
struct Format {
    details: String,
    state: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    id: u64,
    hosts: Vec<String>,
    format: Option<Format>,
}

/// Creates the config directory and default configuration file
fn create_config(path: &Path, filename: &str) -> std::io::Result<()> {
    println!("creating directory at '{:?}'", path);
    fs::create_dir_all(path)?;

    println!("creating default config file");
    let mut config_file = fs::File::create(path.join(filename))?;

    let config = Config {
        id: DISCORD_ID,
        hosts: [DEFAULT_HOST.to_string()].to_vec(),
        format: Some(Format {
            details: DETAILS_FORMAT.to_string(),
            state: STATE_FORMAT.to_string(),
        }),
    };

    config_file.write_all(toml::to_string(&config).unwrap().as_bytes())?;
    Ok(())
}

/// loads the configuration file contents.
/// If the file does not exist it is created.
fn load_config() -> std::io::Result<String> {
    let path = config_dir().unwrap().join(Path::new("discord-rpc"));
    let filename = "config.toml";

    if !path.exists() {
        create_config(&path, filename).expect("Failed to create config file");
    }

    let file = fs::File::open(path.join(filename))?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

/// Cycles through each MPD host and
/// returns the first one which is playing,
/// or none if one is not found.
fn try_get_mpd_conn(hosts: &[String]) -> Option<MPDClient> {
    for host in hosts {
        if let Ok(mut conn) = MPDClient::connect(host) {
            let state = conn.status().unwrap().state;
            if state == State::Play {
                return Some(conn);
            }
        }
    }

    None
}

/// Attempts to find a playing MPD host every 5
/// seconds until one is found
fn idle(hosts: &[String]) -> MPDClient {
    println!("Entering idle mode");

    loop {
        let conn_wrapper = try_get_mpd_conn(hosts);

        if conn_wrapper.is_some() {
            println!("Exiting idle mode");
            return conn_wrapper.unwrap();
        }

        thread::sleep(time::Duration::from_secs(IDLE_TIME));
    }
}

/// Formats a duration given in seconds
/// into hh:mm
fn format_time(time: i64) -> String {
    let seconds = (time as f64 % 60.0).round();
    let minutes = ((time as f64 % 3600.0) / 60.0).round();

    format!("{:0>2}:{:0>2}", minutes, seconds)
}

/// Converts a string format token value
/// into its respective MPD value.
fn get_token_value(client: &mut MPDClient, song: &Song, token: &str) -> String {
    let s = match token {
        "title" => song.title.as_ref(),
        "album" => song.tags.get("Album"),
        "artist" => song.tags.get("Artist"),
        "date" => song.tags.get("Date"),
        "disc" => song.tags.get("Disc"),
        "genre" => song.tags.get("Genre"),
        "track" => song.tags.get("Track"),
        "duration" => return format_time(client.status().unwrap().duration.unwrap().num_seconds()),
        "elapsed" => return format_time(client.status().unwrap().elapsed.unwrap().num_seconds()),
        _ => return token.to_string(),
    };
    s.cloned().unwrap_or_default()
}

fn main() {
    let config: Config = toml::from_str(load_config().unwrap().as_str()).unwrap();

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

    let mut mpd = idle(hosts);
    let mut drpc = DiscordClient::new(config.id);

    drpc.start();

    let re = Regex::new(r"\$(\w+)").unwrap();

    loop {
        let state = mpd.status().unwrap().state;

        if state == State::Play {
            let song: Song = mpd.currentsong().unwrap().unwrap();

            let details = re.replace_all(details_format, |caps: &Captures| {
                get_token_value(&mut mpd, &song, &caps[1])
            });
            let state = re.replace_all(state_format, |caps: &Captures| {
                get_token_value(&mut mpd, &song, &caps[1])
            });

            // set the activity
            if let Err(why) = drpc.set_activity(|act| {
                act.state(state)
                    .details(details)
                    .assets(|asset| asset.small_image("notes"))
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
