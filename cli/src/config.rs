use anyhow::{Context, Result};
use dirs::config_dir;
use std::fs;
use std::path::PathBuf;

use crate::types::Config;

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            toml::from_str(&content).context("Failed to parse config file")
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(&self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let config_dir = config_dir().context("Failed to find config directory")?;
        Ok(config_dir.join("gemini-cli").join("config.toml"))
    }

    pub fn set(&mut self, key: &str, value: String) -> Result<()> {
        match key {
            "api_url" => self.api_url = Some(value),
            "output_dir" => self.output_dir = Some(PathBuf::from(value)),
            "api_key" => self.api_key = Some(value),
            "email" => self.email = Some(value),
            _ => return Err(anyhow::anyhow!("Unknown config key: {}", key)),
        }
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "api_url" => self.api_url.clone(),
            "output_dir" => self.output_dir.as_ref().map(|p| p.display().to_string()),
            "api_key" => self.api_key.clone(),
            "email" => self.email.clone(),
            _ => None,
        }
    }
}