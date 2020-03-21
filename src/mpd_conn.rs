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
/// into hh:mm
fn format_time(time: i64) -> String {
    let seconds = (time as f64 % 60.0).round();
    let minutes = ((time as f64 % 3600.0) / 60.0).round();

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
        "duration" => return format_time(client.status().unwrap().duration.unwrap().num_seconds()),
        "elapsed" => return format_time(client.status().unwrap().elapsed.unwrap().num_seconds()),
        _ => return token.to_string(),
    };
    s.cloned().unwrap_or_default()
}
