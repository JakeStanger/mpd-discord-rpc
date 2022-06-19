use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::TcpStream;

use discord_rpc_client::models::ActivityTimestamps;
use mpd_client::commands::responses::{PlayState, Song, Status};
use mpd_client::{commands, Client as MPDClient, Tag};

/// Cycles through each MPD host and
/// returns the first one which is playing,
/// or none if one is not found.
pub(crate) async fn try_get_mpd_conn(hosts: &[String]) -> Option<MPDClient> {
    for host in hosts {
        let connection = TcpStream::connect(host).await;
        match connection {
            Ok(conn) => match MPDClient::connect(conn).await {
                Ok(conn) => {
                    let client = conn.0;
                    let state = get_status(&client).await.state;
                    if state == PlayState::Playing {
                        return Some(client);
                    }
                }
                Err(why) => eprintln!("Error connecting to {}: {}", host, why),
            },
            Err(why) => eprintln!("Error connecting to {}: {}", host, why),
        }
    }

    None
}

/// Formats a duration given in seconds
/// in hh:mm format
fn format_time(time: u64) -> String {
    let minutes = (time / 60) % 60;
    let seconds = time % 60;

    format!("{:0>2}:{:0>2}", minutes, seconds)
}

/// Converts a string format token value
/// into its respective MPD value.
pub(crate) async fn get_token_value(client: &mut MPDClient, song: &Song, token: &str) -> String {
    let s = match token {
        "title" => song.title(),
        "album" => try_get_first_tag(song.tags.get(&Tag::Album)),
        "artist" => try_get_first_tag(song.tags.get(&Tag::Artist)),
        "date" => try_get_first_tag(song.tags.get(&Tag::Date)),
        "disc" => try_get_first_tag(song.tags.get(&Tag::Disc)),
        "genre" => try_get_first_tag(song.tags.get(&Tag::Genre)),
        "track" => try_get_first_tag(song.tags.get(&Tag::Track)),
        "duration" => return format_time(get_duration(&get_status(client).await)),
        "elapsed" => return format_time(get_elapsed(&get_status(client).await)),
        _ => return token.to_string(),
    };
    s.unwrap_or_default().to_string()
}

/// Gets the activity timestamp based off the current song elapsed/remaining
pub(crate) async fn get_timestamp(client: &mut MPDClient, mode: &str) -> ActivityTimestamps {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let status = get_status(client).await;

    let elapsed = get_elapsed(&status);

    let timestamps = ActivityTimestamps::new();

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
pub(crate) async fn get_status(client: &MPDClient) -> Status {
    client
        .command(commands::Status)
        .await
        .expect("Failed to get MPD server status")
}

/// Attempts to read the first value for a tag
/// (since the MPD client returns a vector of tags, or None)
pub(crate) fn try_get_first_tag(vec: Option<&Vec<String>>) -> Option<&str> {
    match vec {
        Some(vec) => vec.first().map(|val| val.as_str()),
        None => None,
    }
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
