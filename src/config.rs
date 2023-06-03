use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub servers: Vec<Server>,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_file = match get_config_file() {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub name: String,
    pub url: String,
}

fn get_config_file() -> Option<PathBuf> {
    let config_dir = match dirs::config_dir() {
        Some(d) => d,
        None => return None,
    };
    let config_file = config_dir.join("jellything/config.toml");

    if !config_file
        .try_exists()
        .expect("Error checking if config file exists.")
    {
        return None;
    }

    Some(config_file)
}
