use crate::defaults::{
    DEFAULT_HOST, DETAILS_FORMAT, DISCORD_ID, LARGE_IMAGE, LARGE_TEXT, SMALL_IMAGE, SMALL_TEXT,
    STATE_FORMAT, TIMESTAMP_MODE,
};
use dirs::{config_dir, home_dir};
use merge::Merge;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Merge, Clone)]
pub struct Format {
    pub(crate) details: Option<String>,
    pub(crate) state: Option<String>,
    // 'elapsed', 'left', or 'off'
    pub(crate) timestamp: Option<String>,
    pub(crate) large_image: Option<String>,
    pub(crate) small_image: Option<String>,
    pub(crate) large_text: Option<String>,
    pub(crate) small_text: Option<String>,
}

#[derive(Serialize, Deserialize, Merge, Clone)]
pub struct Config {
    pub(crate) id: Option<u64>,
    pub(crate) hosts: Option<Vec<String>>,
    pub(crate) format: Option<Format>,
}

impl Config {
    /// Merges the user's config into the default config
    fn merge_custom(mut self, other: Config) -> Self {
        self.merge(other.clone());
        let mut format = self.format.unwrap();
        format.merge(other.format.unwrap());
        self.format = Some(format);
        self
    }
}

impl Default for Format {
    fn default() -> Self {
        Format {
            details: Some(DETAILS_FORMAT.to_string()),
            state: Some(STATE_FORMAT.to_string()),
            timestamp: Some(TIMESTAMP_MODE.to_string()),
            large_image: Some(LARGE_IMAGE.to_string()),
            small_image: Some(SMALL_IMAGE.to_string()),
            large_text: Some(LARGE_TEXT.to_string()),
            small_text: Some(SMALL_TEXT.to_string()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            id: Some(DISCORD_ID),
            hosts: Some(vec![DEFAULT_HOST.to_string()]),
            format: Some(Format::default()),
        }
    }
}

impl Config {
    /// Gets the path to the config directory
    fn get_dir_path() -> PathBuf {
        let config_dir = config_dir();

        if let Some(config_dir) = config_dir {
            config_dir.join(Path::new("discord-rpc"))
        } else {
            home_dir()
                .expect("Failed to get user config or home directory, cannot create config file")
                .join(".discord-rpc")
        }
    }

    /// Creates the config directory and default configuration file
    fn create(path: &Path, filename: &str) -> std::io::Result<()> {
        println!("creating directory at '{:?}'", path);
        fs::create_dir_all(path)?;

        println!("creating default config file");
        let mut config_file = fs::File::create(path.join(filename))?;

        let config = Config::default();
        config_file.write_all(toml::to_string(&config).unwrap().as_bytes())
    }

    /// loads the configuration file contents.
    /// If the file does not exist it is created.
    pub fn load() -> Config {
        let path = Config::get_dir_path();
        let filename = "config.toml";

        if !path.join(filename).exists() {
            Config::create(&path, filename).expect("Failed to create config file");
        }

        let filepath = path.join(filename);

        let file = fs::File::open(&filepath).unwrap_or_else(|_| {
            panic!("Failed to open file for writing at {}", filepath.display())
        });

        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();

        buf_reader
            .read_to_string(&mut contents)
            .expect("Failed to parse config file as it contains data which is not valid UTF-8");

        toml::from_str::<Config>(contents.as_str())
            .expect("Failed to parse config file as it contains invalid TOML")
            .merge_custom(Config::default())
    }
}
