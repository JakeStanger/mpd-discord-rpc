use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::{TcpStream, UnixStream};

use discord_rpc_client::models::ActivityTimestamps;
use mpd_client::commands::responses::{PlayState, Song, Status};
use mpd_client::raw::MpdProtocolError;
use mpd_client::{commands, Client as MPDClient, Connection, Tag};
use std::os::unix::fs::FileTypeExt;

/// Cycles through each MPD host and
/// returns the first one which is playing,
/// or none if one is not found.
pub(crate) async fn try_get_mpd_conn(hosts: &[String]) -> Option<MPDClient> {
    for host in hosts {
        let connection = if is_unix_socket(host) {
            connect_unix(host).await
        } else {
            connect_tcp(host).await
        };

        match connection {
            Ok(conn) => {
                let client = conn.0;
                let state = get_status(&client).await.state;
                if state == PlayState::Playing {
                    return Some(client);
                }
            }
            Err(why) => eprintln!("Error connecting to {}: {}", host, why),
        }
    }

    None
}

fn is_unix_socket(host: &String) -> bool {
    let path = PathBuf::from(host);
    path.exists()
        && match path.metadata() {
            Ok(metadata) => metadata.file_type().is_socket(),
            Err(_) => false,
        }
}

async fn connect_unix(host: &String) -> Result<Connection, MpdProtocolError> {
    let connection = UnixStream::connect(host)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to unix socket: {}", host));
    MPDClient::connect(connection).await
}

async fn connect_tcp(host: &String) -> Result<Connection, MpdProtocolError> {
    let connection = TcpStream::connect(host)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to unix socket: {}", host));
    MPDClient::connect(connection).await
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
    match token {
        "title" => song.title(),
        "album" => try_get_first_tag(song.tags.get(&Tag::Album)),
        "artist" => try_get_first_tag(song.tags.get(&Tag::Artist)),
        "date" => try_get_first_tag(song.tags.get(&Tag::Date)),
        "disc" => try_get_first_tag(song.tags.get(&Tag::Disc)),
        "genre" => try_get_first_tag(song.tags.get(&Tag::Genre)),
        "track" => try_get_first_tag(song.tags.get(&Tag::Track)),
        "duration" => return format_time(get_duration(&get_status(client).await)),
        "elapsed" => return format_time(get_elapsed(&get_status(client).await)),
        _ => Some(token),
    }
    .unwrap_or("unknown")
    .to_string()
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
