use crate::defaults::{DEFAULT_HOST, DETAILS_FORMAT, DISCORD_ID, STATE_FORMAT, TIMESTAMP_MODE};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Format {
    pub(crate) details: String,
    pub(crate) state: String,
    // 'elapsed', 'left', or 'off'. optional as new feat
    pub(crate) timestamp: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub(crate) id: u64,
    pub(crate) hosts: Vec<String>,
    pub(crate) format: Option<Format>,
}

impl Config {
    /// Creates the config directory and default configuration file
    fn create(path: &Path, filename: &str) -> std::io::Result<()> {
        println!("creating directory at '{:?}'", path);
        fs::create_dir_all(path)?;

        println!("creating default config file");
        let mut config_file = fs::File::create(path.join(filename))?;

        let config = Config {
            id: DISCORD_ID,
            hosts: [DEFAULT_HOST.to_string()].to_vec(),
            format: Some(Format {
                details: DETAILS_FORMAT.to_string(),
                state: STATE_FORMAT.to_string(),
                timestamp: Some(TIMESTAMP_MODE.to_string()),
            }),
        };

        config_file.write_all(toml::to_string(&config).unwrap().as_bytes())
    }

    /// loads the configuration file contents.
    /// If the file does not exist it is created.
    pub fn load() -> Config {
        let path = config_dir().unwrap().join(Path::new("discord-rpc"));
        let filename = "config.toml";

        if !path.exists() {
            Config::create(&path, filename).expect("Failed to create config file");
        }

        let file = fs::File::open(path.join(filename)).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        toml::from_str(contents.as_str()).unwrap()
    }
}
