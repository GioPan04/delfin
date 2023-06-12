use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub device_id: String,
    pub servers: Vec<Server>,
    #[serde(default)]
    pub video_player: VideoPlayerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            device_id: Uuid::new_v4().to_string(),
            servers: Vec::default(),
            video_player: VideoPlayerConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoPlayerConfig {
    pub position_update_frequency: usize,
}

impl Default for VideoPlayerConfig {
    fn default() -> Self {
        Self {
            position_update_frequency: 10,
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
        let config_file = match get_config_file() {
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

fn get_config_file() -> Option<PathBuf> {
    let config_dir = match dirs::config_dir() {
        Some(d) => d,
        None => return None,
    };
    let config_file = config_dir.join("jellything/config.toml");

    Some(config_file)
}

fn get_config_file_exists() -> Option<PathBuf> {
    let config_file = match get_config_file() {
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
