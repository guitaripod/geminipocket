use anyhow::Result;
use colored::*;
use std::io::{self, Write};

use crate::api::GeminiClient;
use crate::types::Config;


pub async fn handle_register(client: &GeminiClient, config: &mut Config) -> Result<()> {
    print!("{}: ", "Email".bold());
    io::stdout().flush()?;
    let mut email = String::new();
    io::stdin().read_line(&mut email)?;
    let email = email.trim();

    print!("{}: ", "Password".bold());
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim();

    println!("{}", "Registering...".cyan());

    match client.register(email, password).await {
        Ok(response) => {
            if response.success {
                if let Some(api_key) = response.api_key {
                    config.api_key = Some(api_key.clone());
                    config.email = Some(email.to_string());
                    config.save()?;

                    println!("{}", "✓ Registration successful!".green());
                    println!("{}: {}", "API Key".bold(), api_key.yellow());
                    println!("{}", "Your API key has been saved to the config.".cyan());
                }
            } else {
                println!("{}", "✗ Registration failed".red());
                if let Some(error) = response.error {
                    println!("{}: {}", "Error".bold(), error.red());
                }
            }
        }
        Err(e) => {
            println!("{}", "✗ Registration failed".red());
            println!("{}: {}", "Error".bold(), e.to_string().red());
        }
    }

    Ok(())
}

pub async fn handle_login(client: &GeminiClient, config: &mut Config) -> Result<()> {
    print!("{}: ", "Email".bold());
    io::stdout().flush()?;
    let mut email = String::new();
    io::stdin().read_line(&mut email)?;
    let email = email.trim();

    print!("{}: ", "Password".bold());
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim();

    println!("{}", "Logging in...".cyan());

    match client.login(email, password).await {
        Ok(response) => {
            if response.success {
                if let Some(api_key) = response.api_key {
                    config.api_key = Some(api_key.clone());
                    config.email = Some(email.to_string());
                    config.save()?;

                    println!("{}", "✓ Login successful!".green());
                    println!("{}: {}", "API Key".bold(), api_key.yellow());
                    println!("{}", "Your API key has been saved to the config.".cyan());
                }
            } else {
                println!("{}", "✗ Login failed".red());
                if let Some(error) = response.error {
                    println!("{}: {}", "Error".bold(), error.red());
                }
            }
        }
        Err(e) => {
            println!("{}", "✗ Login failed".red());
            println!("{}: {}", "Error".bold(), e.to_string().red());
        }
    }

    Ok(())
}

pub fn handle_logout(config: &mut Config) -> Result<()> {
    config.api_key = None;
    config.email = None;
    config.save()?;

    println!("{}", "✓ Logged out successfully!".green());
    println!("{}", "API key has been removed from config.".cyan());

    Ok(())
}

pub fn handle_status(config: &Config) -> Result<()> {
    if let Some(email) = &config.email {
        println!("{}", "✓ Logged in".green());
        println!("{}: {}", "Email".bold(), email.cyan());
        if config.api_key.is_some() {
            println!("{}", "API Key: Configured".green());
        }
    } else {
        println!("{}", "✗ Not logged in".yellow());
        println!("{}", "Use 'geminipocket-cli auth login' to log in.".cyan());
    }

    Ok(())
}