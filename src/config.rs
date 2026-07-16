use serde::{Deserialize, Deserializer, Serialize};
use universal_config::ConfigLoader;

#[derive(Serialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AlbumArtMode {
    #[default]
    Remote,
    Local,
    PreferLocal,
    PreferRemote,
    None,
}

impl<'de> Deserialize<'de> for AlbumArtMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "remote" => Ok(AlbumArtMode::Remote),
            "local" => Ok(AlbumArtMode::Local),
            "prefer_local" => Ok(AlbumArtMode::PreferLocal),
            "prefer_remote" => Ok(AlbumArtMode::PreferRemote),
            "none" => Ok(AlbumArtMode::None),
            other => Err(serde::de::Error::custom(format!(
                "unknown album_art mode: {}",
                other
            ))),
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum TimestampMode {
    Elapsed,
    Left,
    Off,
    #[default]
    Both,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum DisplayType {
    Name,
    #[default]
    State,
    Details,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Format {
    #[serde(default = "default_details_format")]
    pub details: String,
    #[serde(default = "default_state_format")]
    pub state: String,
    #[serde(default)]
    pub timestamp: TimestampMode,
    #[serde(default = "default_image")]
    pub large_image: String,
    #[serde(default = "default_image")]
    pub small_image: String,
    #[serde(default)]
    pub large_text: String,
    #[serde(default)]
    pub small_text: String,
    #[serde(default)]
    pub display_type: DisplayType,
    #[serde(default)]
    pub album_art: AlbumArtMode,
    #[serde(default)]
    pub button1_text: String,
    #[serde(default)]
    pub button1_link: String,
    #[serde(default)]
    pub button2_text: String,
    #[serde(default)]
    pub button2_link: String,
}

impl Default for Format {
    fn default() -> Self {
        Self {
            details: default_details_format(),
            state: default_state_format(),
            timestamp: TimestampMode::default(),
            large_image: default_image(),
            small_image: default_image(),
            large_text: String::new(),
            small_text: String::new(),
            display_type: DisplayType::default(),
            album_art: AlbumArtMode::default(),
            button1_text: String::new(),
            button1_link: String::new(),
            button2_text: String::new(),
            button2_link: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_discord_id")]
    pub id: u64,
    #[serde(default = "default_mpd_hosts")]
    pub hosts: Vec<String>,
    #[serde(default)]
    pub format: Format,
    #[serde(default = "default_music_directory")]
    pub music_directory: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            id: default_discord_id(),
            hosts: default_mpd_hosts(),
            format: Format::default(),
            music_directory: default_music_directory(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let loader =
            ConfigLoader::new("discord-rpc").with_formats(&[universal_config::Format::Toml]);

        let mut cfg = loader.find_and_load().unwrap_or_else(|_| {
            let cfg = Self::default();
            loader
                .save(&cfg, &universal_config::Format::Toml)
                .expect("Failed to create default config file");
            cfg
        });

        // Clean up empty music directory parameter
        if let Some(ref s) = cfg.music_directory
            && s.trim().is_empty()
        {
            cfg.music_directory = None;
        }

        cfg
    }
}

fn default_details_format() -> String {
    "$title".to_string()
}

fn default_state_format() -> String {
    "$artist / $album".to_string()
}

fn default_image() -> String {
    "notes".to_string()
}

const fn default_discord_id() -> u64 {
    677226551607033903
}

fn default_mpd_hosts() -> Vec<String> {
    vec!["localhost:6600".to_string()]
}

fn default_music_directory() -> Option<String> {
    dirs::home_dir()
        .map(|home| home.join("Music").to_string_lossy().into_owned())
        .map(Some)
        .unwrap_or(None)
}
