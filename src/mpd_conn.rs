use std::time::{SystemTime, UNIX_EPOCH};

use discord_rpc_client::models::ActivityTimestamps;
use mpd::{Client as MPDClient, Song, State};

/// Cycles through each MPD host and
/// returns the first one which is playing,
/// or none if one is not found.
pub(crate) fn try_get_mpd_conn(hosts: &[String]) -> Option<MPDClient> {
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
        "duration" => return format_time(client.status().unwrap().time.unwrap().1.num_seconds()),
        "elapsed" => return format_time(client.status().unwrap().elapsed.unwrap().num_seconds()),
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
    let status = client.status().unwrap();

    match mode {
        "left" => {
            let remaining =
                status.duration.unwrap().num_seconds() - status.elapsed.unwrap().num_seconds();
            timestamps.end(current_time + remaining as u64)
        }
        "off" => timestamps,
        _ => timestamps.start(current_time - status.elapsed.unwrap().num_seconds() as u64),
    }
}
