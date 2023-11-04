pub mod video_player_config;

use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sys_locale::get_locale;
use unic_langid::{langid, LanguageIdentifier};
use uuid::Uuid;

use self::video_player_config::VideoPlayerConfig;

fn get_default_language() -> LanguageIdentifier {
    get_locale()
        .and_then(|l| l.parse().ok())
        .unwrap_or_else(|| {
            println!("Error parsing system locale, defaulting to en-US");
            langid!("en-US")
        })
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default = "get_default_language")]
    pub language: LanguageIdentifier,
    pub device_id: String,
    pub servers: Vec<Server>,
    #[serde(default)]
    pub video_player: VideoPlayerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: get_default_language(),
            device_id: Uuid::new_v4().to_string(),
            servers: Vec::default(),
            video_player: VideoPlayerConfig::default(),
        }
    }
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_file = match get_config_file_exists() {
            Some(f) => f,
            None => return Ok(Self::default()),
        };
        let config = fs::read_to_string(config_file)?;
        Ok(toml::from_str(&config)?)
    }

    pub fn save(&self) -> Result<()> {
        let config_file = match get_config_file(true) {
            Some(f) => f,
            None => return Err(anyhow!("Error getting config file")),
        };
        let toml = toml::to_string_pretty(&self)?;
        fs::write(config_file, toml)?;
        Ok(())
    }

    pub fn to_toml(&self) -> Result<String> {
        Ok(toml::to_string_pretty(&self)?)
    }
}

fn get_config_file(create_dir: bool) -> Option<PathBuf> {
    let config_dir = match dirs::config_dir() {
        Some(d) => d,
        None => return None,
    };

    let dir = config_dir.join("delfin");

    if create_dir {
        fs::create_dir_all(&dir).expect("Error creating config directory");
    }

    let config_file = dir.join("config.toml");

    Some(config_file)
}

fn get_config_file_exists() -> Option<PathBuf> {
    let config_file = match get_config_file(false) {
        Some(f) => f,
        None => return None,
    };

    if !config_file
        .try_exists()
        .expect("Error checking if config file exists.")
    {
        return None;
    }

    Some(config_file)
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Server {
    pub id: String,
    pub url: String,
    pub name: String,
    pub accounts: Vec<Account>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    // TODO: move to keyring
    pub access_token: String,
}
