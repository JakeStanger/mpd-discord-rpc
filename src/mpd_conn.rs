use crate::config::TimestampMode;
use discord_rpc_client::models::ActivityTimestamps;
use mpd_client::responses::{Song, Status};
use mpd_client::tag::Tag;
use std::time::{SystemTime, UNIX_EPOCH};

/// Formats a duration given in seconds
/// in hh:mm format
fn format_time(time: u64) -> String {
    let minutes = (time / 60) % 60;
    let seconds = time % 60;

    format!("{minutes:0>2}:{seconds:0>2}")
}

/// Converts a string format token value
/// into its respective MPD value.
pub fn get_token_value(song: &Song, status: &Status, token: &str) -> String {
    match token {
        "title" => song.title(),
        "album" => try_get_first_tag(song.tags.get(&Tag::Album)),
        "artist" => try_get_first_tag(song.tags.get(&Tag::Artist)),
        "albumartist" => try_get_first_tag(song.tags.get(&Tag::AlbumArtist)),
        "date" => try_get_first_tag(song.tags.get(&Tag::Date)),
        "disc" => try_get_first_tag(song.tags.get(&Tag::Disc)),
        "genre" => try_get_first_tag(song.tags.get(&Tag::Genre)),
        "track" => try_get_first_tag(song.tags.get(&Tag::Track)),
        "duration" => return format_time(get_duration(status)),
        "elapsed" => return format_time(get_elapsed(status)),
        _ => Some(token),
    }
    .unwrap_or("unknown")
    .to_string()
}

/// Gets the activity timestamp based off the current song elapsed/remaining
pub fn get_timestamp(status: &Status, mode: TimestampMode) -> ActivityTimestamps {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get system time")
        .as_secs();

    let elapsed = get_elapsed(status);

    let timestamps = ActivityTimestamps::new();

    match mode {
        TimestampMode::Left => {
            let duration = get_duration(status);

            let remaining = duration - elapsed;
            timestamps.end(current_time + remaining)
        }
        TimestampMode::Off => timestamps,
        TimestampMode::Elapsed => timestamps.start(current_time - elapsed),
    }
}

/// Attempts to read the first value for a tag
/// (since the MPD client returns a vector of tags, or None)
pub fn try_get_first_tag(vec: Option<&Vec<String>>) -> Option<&str> {
    vec.and_then(|vec| vec.first().map(String::as_str))
}

/// Gets the duration of the current song
fn get_duration(status: &Status) -> u64 {
    status
        .duration
        .expect("Failed to get duration from MPD status")
        .as_secs()
}

/// Gets the elapsed time of the current song
fn get_elapsed(status: &Status) -> u64 {
    status
        .elapsed
        .expect("Failed to get elapsed time from MPD status")
        .as_secs()
}
