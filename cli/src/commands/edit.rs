use anyhow::Result;
use colored::*;
use std::path::{Path, PathBuf};

use crate::api::GeminiClient;
use crate::utils::save_image;

pub async fn handle_edit(
    client: &GeminiClient,
    image_path: &PathBuf,
    prompt: &str,
    output_dir: Option<&Path>,
    filename: Option<&str>,
    save_to_current: bool,
) -> Result<()> {
    if !image_path.exists() {
        eprintln!("{} Image file not found: {}", "✗".red(), image_path.display());
        return Ok(());
    }

    println!(
        "{} {} with prompt: {}",
        "Editing".bold(),
        image_path.display().to_string().cyan(),
        prompt.italic()
    );

    match client.edit_image(image_path, prompt).await {
        Ok(response) => {
            if response.success {
                if let Some(image_data) = response.image {
                    let output_path =
                        save_image(&image_data, output_dir, filename, save_to_current)?;
                    println!(
                        "{} Edited image saved to: {}",
                        "✓".green(),
                        output_path.display().to_string().bold()
                    );
                }
            } else {
                eprintln!(
                    "{} Edit failed: {}",
                    "✗".red(),
                    response.error.unwrap_or_else(|| "Unknown error".to_string())
                );
            }
        }
        Err(e) => eprintln!("{} Error: {}", "✗".red(), e),
    }

    Ok(())
}