use anyhow::Result;
use colored::*;

use crate::api::GeminiClient;

pub async fn handle_info(client: &GeminiClient) -> Result<()> {
    match client.info().await {
        Ok(info) => {
            println!("{}", "API Information".bold());
            println!("  {}: {}", "Name".cyan(), info.name);
            println!("  {}: {}", "Version".cyan(), info.version);
            println!("  {}:", "Endpoints".cyan());

            if let Some(endpoints) = info.endpoints.as_object() {
                for (key, value) in endpoints {
                    println!("    • {}: {}", key.yellow(), value.as_str().unwrap_or(""));
                }
            }
        }
        Err(_) => {
            println!("{} Failed to get API info", "✗".red());
        }
    }
    Ok(())
}