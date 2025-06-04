use serde::{Deserialize, Serialize};
use std::default::Default;
use universal_config::ConfigLoader;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TimestampMode {
    Elapsed,
    Left,
    Off,
    Both,
}

impl Default for TimestampMode {
    fn default() -> Self {
        Self::Both
    }
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
}

impl Default for Config {
    fn default() -> Self {
        Self {
            id: default_discord_id(),
            hosts: default_mpd_hosts(),
            format: Format::default(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let loader =
            ConfigLoader::new("discord-rpc").with_formats(&[universal_config::Format::Toml]);

        loader.find_and_load().unwrap_or_else(|_| {
            let cfg = Self::default();
            loader
                .save(&cfg, &universal_config::Format::Toml)
                .expect("Failed to create default config file");
            cfg
        })
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
