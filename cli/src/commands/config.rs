use anyhow::Result;
use clap::Subcommand;
use colored::*;

use crate::types::Config;

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Update a configuration value
    Set {
        /// Config key: 'api_url' or 'output_dir'
        #[arg(value_name = "KEY")]
        key: String,
        
        /// New value for the key
        #[arg(value_name = "VALUE")]
        value: String,
    },
    
    /// Show a configuration value
    Get {
        /// Config key to retrieve
        #[arg(value_name = "KEY")]
        key: String,
    },
    
    /// Show all configuration values
    List,
}

pub fn handle_config(action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Set { key, value } => {
            let mut config = Config::load()?;
            if let Err(e) = config.set(&key, value.clone()) {
                eprintln!("{} {}", "✗".red(), e);
                eprintln!("\n{}: api_url, output_dir", "Valid keys".cyan());
                return Ok(());
            }
            config.save()?;
            println!("{} Config updated: {} = {}", "✓".green(), key.cyan(), value);
        }
        ConfigAction::Get { key } => {
            let config = Config::load()?;
            if let Some(value) = config.get(&key) {
                println!("{}: {}", key.cyan(), value);
            } else {
                println!("{}: {}", key.cyan(), "(not set)".dimmed());
                println!("\n{}: api_url, output_dir", "Valid keys".cyan());
            }
        }
        ConfigAction::List => {
            let config = Config::load()?;
            println!("{}", "Configuration:".bold());
            println!(
                "  {}: {}",
                "api_url".cyan(),
                config.api_url.as_deref().unwrap_or("(not set)")
            );
            println!(
                "  {}: {}",
                "output_dir".cyan(),
                config
                    .output_dir
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "(not set)".to_string())
            );
        }
    }
    Ok(())
}