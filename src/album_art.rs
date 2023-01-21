use crate::mpd_conn::try_get_first_tag;
use mpd_client::responses::Song;
use mpd_client::tag::Tag;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct SearchResult {
    releases: Vec<Release>,
}

#[derive(Deserialize)]
struct Release {
    id: String,
}

pub struct AlbumArtClient {
    release_cache: HashMap<(String, String), String>,
}

impl AlbumArtClient {
    pub fn new() -> AlbumArtClient {
        let release_cache = HashMap::new();
        AlbumArtClient { release_cache }
    }

    /// Searches for a release on MusicBrainz
    /// Returns its ID if one is found.
    fn find_release(&mut self, artist: String, album: String) -> Option<String> {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

        let query = format!("artist:{} AND release:{}", &artist, &album);

        let cache_key = (artist, album);
        if self.release_cache.contains_key(&cache_key) {
            return Some(self.release_cache.get(&cache_key).unwrap().to_string());
        }

        let url = format!(
            "https://musicbrainz.org/ws/2/release/?query={}&limit=1",
            query
        );

        let client = Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .expect("Failed to create HTTP client");

        let response = client.get(&url).header("Accept", "application/json").send();

        if let Ok(response) = response {
            if response.status() != 200 {
                return None;
            }

            let response = response
                .json::<SearchResult>()
                .expect("Received response from MusicBrainz in unexpected format");

            let id = response.releases.first().map(|release| release.id.clone());

            match id {
                Some(id) => {
                    self.release_cache.insert(cache_key, id.clone());
                    Some(id)
                }
                None => None,
            }
        } else {
            None
        }
    }

    /// Attempts to get the URL to the current album's front cover
    /// by fetching it from MusicBrainz.
    ///
    /// Uses MPD's internal MusicBrainz album ID tag if its set,
    /// otherwise falls back to searching.
    pub fn get_album_art_url(&mut self, song: Song) -> Option<String> {
        let mb_album_id = match try_get_first_tag(song.tags.get(&Tag::MusicBrainzReleaseId)) {
            Some(id) => Some(id.to_string()),
            None => {
                let tags = song.tags;
                let artist = try_get_first_tag(tags.get(&Tag::Artist));
                let album = try_get_first_tag(tags.get(&Tag::Album));

                match (artist, album) {
                    (Some(artist), Some(album)) => {
                        self.find_release(artist.to_string(), album.to_string())
                    }
                    _ => None,
                }
            }
        };

        mb_album_id.map(|id| format!("https://coverartarchive.org/release/{}/front-250", id))
    }
}
