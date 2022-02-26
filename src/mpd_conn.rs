use std::time::{SystemTime, UNIX_EPOCH};

use discord_rpc_client::models::ActivityTimestamps;
use mpd::{Client as MPDClient, Song, State, Status};

/// Cycles through each MPD host and
/// returns the first one which is playing,
/// or none if one is not found.
pub(crate) fn try_get_mpd_conn(hosts: &[String]) -> Option<MPDClient> {
    for host in hosts {
        match MPDClient::connect(host) {
            Ok(mut conn) => {
                let state = get_status(&mut conn).state;
                if state == State::Play {
                    return Some(conn);
                }
            }
            Err(why) => eprintln!("Error connecting to {}: {}", host, why),
        }
    }

    None
}

/// Formats a duration given in seconds
// in hh:mm format
fn format_time(time: i64) -> String {
    let minutes = (time / 60) % 60;
    let seconds = time % 60;

    format!("{:0>2}:{:0>2}", minutes, seconds)
}

/// Converts a string format token value
/// into its respective MPD value.
pub(crate) fn get_token_value(client: &mut MPDClient, song: &Song, token: &str) -> String {
    let s = match token {
        "title" => song.title.as_ref(),
        "album" => song.tags.get("Album"),
        "artist" => song.tags.get("Artist"),
        "date" => song.tags.get("Date"),
        "disc" => song.tags.get("Disc"),
        "genre" => song.tags.get("Genre"),
        "track" => song.tags.get("Track"),
        "duration" => return format_time(get_time(&get_status(client))),
        "elapsed" => return format_time(get_elapsed(&get_status(client))),
        _ => return token.to_string(),
    };
    s.cloned().unwrap_or_default()
}

pub(crate) fn get_timestamp(
    client: &mut MPDClient,
    timestamps: ActivityTimestamps,
    mode: &str,
) -> ActivityTimestamps {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let status = get_status(client);

    let elapsed = get_elapsed(&status);

    match mode {
        "left" => {
            let duration = get_duration(&status);

            let remaining = duration - elapsed;
            timestamps.end(current_time + remaining as u64)
        }
        "off" => timestamps,
        _ => timestamps.start(current_time - elapsed as u64),
    }
}

/// Gets MPD server status.
/// Panics on error.
pub(crate) fn get_status(client: &mut MPDClient) -> Status {
    client.status().expect("Failed to get MPD server status")
}

fn get_time(status: &Status) -> i64 {
    status
        .time
        .expect("Failed to get duration (time) from MPD status")
        .1
        .num_seconds()
}

fn get_duration(status: &Status) -> i64 {
    status
        .duration
        .expect("Failed to get duration from MPD status")
        .num_seconds()
}

fn get_elapsed(status: &Status) -> i64 {
    status
        .elapsed
        .expect("Failed to get elapsed time from MPD status")
        .num_seconds()
}
