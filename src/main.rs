use std::{thread, time};
use std::fs;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

use dirs::config_dir;
use discord_rpc_client::Client as DiscordClient;
use mpd::{Client as MPDClient, Song, State};
use toml::Value;

const IDLE_TIME: u64 = 5;
const ACTIVE_TIME: u64 = 1;

static EMPTY: String = String::new();

/// Creates the config directory and default configuration file
fn create_config(path: &PathBuf, filename: &str) -> std::io::Result<()> {
    println!("creating directory at '{:?}'", path);
    fs::create_dir_all(path)?;

    println!("creating default config file");
    let mut config = fs::File::create(path.join(filename))?;
    config.write_all(b"id = 677226551607033903\nhosts = ['localhost:6600']")?;
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

fn main() {
    let config = load_config().unwrap().parse::<Value>().unwrap();

    let app_id = config["id"].as_integer().unwrap() as u64;
    let hosts: &Vec<String> = &config["hosts"].as_array().unwrap().iter()
        .map(|val| val.as_str().unwrap().to_string())
        .collect();

    let mut mpd = idle(hosts);
    let mut drpc = DiscordClient::new(app_id);

    drpc.start();

    loop {
        let state = mpd.status().unwrap().state;

        if state == State::Play {
            let song: Song = mpd.currentsong().unwrap().unwrap();

            let title = song.title.unwrap();
            let album = song.tags.get("Album").unwrap_or(&EMPTY);
            let artist = song.tags.get("Artist").unwrap_or(&EMPTY);

            // set the activity
            if let Err(why) = drpc.set_activity(|act| act.state(format!("{} / {}", artist, album))
                .details(title)
                .assets(|asset| asset.small_image("notes"))) {
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