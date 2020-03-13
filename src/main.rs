use std::fs;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::{thread, time};

use dirs::config_dir;
use discord_rpc_client::Client as DiscordClient;
use mpd::{Client as MPDClient, Song, State};
use regex::{Captures, Regex};
use toml::Value;

const IDLE_TIME: u64 = 5;
const ACTIVE_TIME: u64 = 1;

const DISCORD_ID: &str = "677226551607033903";
const DEFAULT_HOST: &str = "localhost:6600";
const DETAILS_FORMAT: &str = "$title";
const STATE_FORMAT: &str = "$artist / $album";

/// Creates the config directory and default configuration file
fn create_config(path: &Path, filename: &str) -> std::io::Result<()> {
    println!("creating directory at '{:?}'", path);
    fs::create_dir_all(path)?;

    println!("creating default config file");
    let mut config = fs::File::create(path.join(filename))?;
    config.write_all(
        format!(
            "id = {}
hosts = ['{}']

[format]
details = \"{}\"
state = \"{}\"
    ",
            DISCORD_ID, DEFAULT_HOST, DETAILS_FORMAT, STATE_FORMAT
        )
        .as_bytes(),
    )?;
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

fn format_time(time: i64) -> String {
    let seconds = (time as f64 % 60.0).round();
    let minutes = ((time as f64 % 3600.0) / 60.0).round();

    format!("{:0>2}:{:0>2}", minutes, seconds)
}

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
    let config = load_config().unwrap().parse::<Value>().unwrap();

    let app_id = config["id"].as_integer().unwrap() as u64;
    let hosts: &Vec<String> = &config["hosts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|val| val.as_str().unwrap().to_string())
        .collect();

    let format_options = &config.get("format");

    let details_format = match &format_options {
        Some(options) => options.as_table().unwrap()["details"]
            .as_str()
            .unwrap_or(DETAILS_FORMAT),
        None => DETAILS_FORMAT,
    };

    let state_format = match &format_options {
        Some(options) => options.as_table().unwrap()["state"]
            .as_str()
            .unwrap_or(STATE_FORMAT),
        None => STATE_FORMAT,
    };

    let mut mpd = idle(hosts);
    let mut drpc = DiscordClient::new(app_id);

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

            mpd = idle(&hosts);
        }

        // sleep for 1 sec to not hammer the mpd and rpc servers
        thread::sleep(time::Duration::from_secs(ACTIVE_TIME));
    }
}
