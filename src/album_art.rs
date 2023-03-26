use crate::mpd_conn::try_get_first_tag;
use mpd_client::responses::Song;
use mpd_client::tag::Tag;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct SearchResult {
    release_groups: Vec<ReleaseGroup>,
}

#[derive(Deserialize, Debug)]
struct ReleaseGroup {
    id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Release {
    id: String,
    release_group: ReleaseGroup,
    cover_art_archive: ReleaseCoverArt,
}

#[derive(Deserialize, Debug)]
struct ReleaseCoverArt {
    front: bool,
}

#[derive(Debug, Copy, Clone)]
enum Type {
    Release,
    ReleaseGroup,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Release => "release",
                Self::ReleaseGroup => "release-group",
            }
        )
    }
}

pub struct AlbumArtClient {
    release_group_cache: HashMap<(String, String), (String, Type)>,
    client: Client,
}

impl AlbumArtClient {
    pub fn new() -> Self {
        let release_group_cache = HashMap::new();

        let mut header_map = HeaderMap::new();
        header_map.insert(
            "accept",
            HeaderValue::from_str("application/json").expect("Failed to parse content type"),
        );

        let client = Client::builder()
            .user_agent(APP_USER_AGENT)
            .default_headers(header_map)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            release_group_cache,
            client,
        }
    }

    /// Looks up a release by its UUID on MusicBrainz.
    /// If the release has a cover, returns the ID of that record.
    /// If not, returns the ID of its release group.
    async fn get_record_id(&self, release_id: &str) -> Option<(String, Type)> {
        let url = format!("https://musicbrainz.org/ws/2/release/{release_id}?inc=release-groups");

        let response = self.client.get(&url).send().await;

        match response {
            Ok(response) if response.status() == 200 => {
                let response = response.json::<Release>().await;
                response.ok().map(|release| {
                    if release.cover_art_archive.front {
                        (release.id, Type::Release)
                    } else {
                        (release.release_group.id, Type::ReleaseGroup)
                    }
                })
            }
            _ => None,
        }
    }

    /// Searches for a release on MusicBrainz
    /// Returns its ID if one is found.
    async fn find_release_group_id(&self, artist: &str, album: &str) -> Option<String> {
        let query = format!("artist:{artist} AND release:{album}");
        let url = format!("https://musicbrainz.org/ws/2/release-group/?query={query}&limit=1",);

        let response = self.client.get(&url).send().await;

        if let Ok(response) = response {
            if response.status() != 200 {
                return None;
            }

            let mut response = response
                .json::<SearchResult>()
                .await
                .expect("Received response from MusicBrainz in unexpected format");

            response.release_groups.pop().map(|rg| rg.id)
        } else {
            None
        }
    }

    fn get_cache_key(song: &Song) -> Option<(String, String)> {
        let tags = &song.tags;
        let artist = try_get_first_tag(tags.get(&Tag::Artist));
        let album = try_get_first_tag(tags.get(&Tag::Album));

        match (artist, album) {
            (Some(artist), Some(album)) => Some((artist.to_string(), album.to_string())),
            _ => None,
        }
    }

    /// Attempts to get the URL to the current album's front cover
    /// by fetching it from MusicBrainz.
    ///
    /// Uses MPD's internal MusicBrainz album ID tag if its set,
    /// otherwise falls back to searching.
    pub async fn get_album_art_url(&mut self, song: Song) -> Option<String> {
        let cache_key = Self::get_cache_key(&song);

        if let Some(cache_key) = cache_key {
            let id = if let Some(id) = self.release_group_cache.remove(&cache_key) {
                Some(id)
            } else {
                let release_id = try_get_first_tag(song.tags.get(&Tag::MusicBrainzReleaseId));
                if let Some(release_id) = release_id {
                    self.get_record_id(release_id).await
                } else {
                    self.find_release_group_id(&cache_key.0, &cache_key.1)
                        .await
                        .map(|id| (id, Type::ReleaseGroup))
                }
            };

            if let Some((id, record_type)) = id {
                self.release_group_cache
                    .insert(cache_key, (id.clone(), record_type));
                Some(format!(
                    "https://coverartarchive.org/{record_type}/{id}/front-250"
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}
