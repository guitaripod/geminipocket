use anyhow::Result;
use colored::*;

use crate::api::GeminiClient;

pub async fn handle_health(client: &GeminiClient) -> Result<()> {
    match client.health().await {
        Ok(health) => {
            println!("{} API is {}", "✓".green(), health.status.green());
            if let Some(timestamp) = health.timestamp {
                let dt = chrono::DateTime::from_timestamp(timestamp as i64 / 1000, 0)
                    .unwrap_or_else(chrono::Utc::now);
                println!("  Last checked: {}", dt.format("%Y-%m-%d %H:%M:%S UTC"));
            }
        }
        Err(_) => {
            println!("{} API health check failed", "✗".red());
        }
    }
    Ok(())
}