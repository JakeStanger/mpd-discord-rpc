use crate::defaults::{
    DEFAULT_HOST,
    DETAILS_FORMAT,
    DISCORD_ID,
    STATE_FORMAT,
    TIMESTAMP_MODE,
    LARGE_IMAGE,
    SMALL_IMAGE,
    LARGE_TEXT,
    SMALL_TEXT,
};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::default::Default;

#[derive(Serialize, Deserialize)]
pub struct Format {
    pub(crate) details: Option<String>,
    pub(crate) state: Option<String>,
    // 'elapsed', 'left', or 'off'. optional as new feat
    pub(crate) timestamp: Option<String>,
    pub(crate) large_image: Option<String>,
    pub(crate) small_image: Option<String>,
    pub(crate) large_text: Option<String>,
    pub(crate) small_text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub(crate) id: Option<u64>,
    pub(crate) hosts: Option<Vec<String>>,
    pub(crate) format: Option<Format>,
}

impl Format {
    fn merge(self, other: Format) -> Self {
        Self {
            details: self.details.or(other.details),
            state: self.state.or(other.state),
            timestamp: self.timestamp.or(other.timestamp),
            large_image: self.large_image.or(other.large_image),
            small_image: self.small_image.or(other.small_image),
            large_text: self.large_text.or(other.large_text),
            small_text: self.small_text.or(other.small_text),
        }
    } 
}

impl Config {
    fn merge(self, other: Config) -> Self {
        Self {
            id: self.id.or(other.id),
            hosts: self.hosts.or(other.hosts),
            format: match self.format {
                Some(format) => Some(format.merge(other.format.unwrap())),
                _ => other.format
            }
        }
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
        let path = config_dir().unwrap().join(Path::new("discord-rpc"));
        let filename = "config.toml";

        if !path.join(filename).exists() {
            Config::create(&path, filename).expect("Failed to create config file");
        }

        let file = fs::File::open(path.join(filename)).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        toml::from_str::<Config>(contents.as_str())
            .unwrap()
            .merge(Config::default())
    }
}
