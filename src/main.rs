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

static EMPTY: String = String::new();

static DISCORD_ID: &str = "677226551607033903";
static DEFAULT_HOST: &str = "localhost:6600";
static DETAILS_FORMAT: &str = "$title";
static STATE_FORMAT: &str = "$artist / $album";

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

fn format_duration(duration: Option<&String>) -> Option<String> {
    if duration.is_some() {
        let time = duration.unwrap().parse::<f32>().unwrap();

        let seconds = (time % 60.0).round();
        let minutes = ((time % 3600.0) / 60.0).round();

        let formatted = format!("{:0>2}:{:0>2}", minutes, seconds);
        return Some(formatted);
    }

    None
}

fn get_token_value(song: &Song, token: &str) -> String {
    match token {
        "title" => song.title.as_ref().unwrap_or(&EMPTY).clone(),
        "album" => song.tags.get("Album").unwrap_or(&EMPTY).clone(),
        "artist" => song.tags.get("Artist").unwrap_or(&EMPTY).clone(),
        "date" => song.tags.get("Date").unwrap_or(&EMPTY).clone(),
        "disc" => song.tags.get("Disc").unwrap_or(&EMPTY).clone(),
        "genre" => song.tags.get("Genre").unwrap_or(&EMPTY).clone(),
        "track" => song.tags.get("Track").unwrap_or(&EMPTY).clone(),
        "duration" => format_duration(song.tags.get("duration")).unwrap_or(String::new()),
        _ => token.to_string(),
    }
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
        None => DETAILS_FORMAT
    };

    let state_format = match &format_options {
        Some(options) => options.as_table().unwrap()["state"]
                .as_str()
                .unwrap_or(STATE_FORMAT),
        None => STATE_FORMAT
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
                get_token_value(&song, &caps[1])
            });
            let state = re.replace_all(state_format, |caps: &Captures| {
                get_token_value(&song, &caps[1])
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
