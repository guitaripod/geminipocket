use anyhow::Result;
use colored::*;
use std::path::Path;

use crate::api::GeminiClient;
use crate::utils::save_image;

pub async fn handle_generate(
    client: &GeminiClient,
    prompt: &str,
    output_dir: Option<&Path>,
    filename: Option<&str>,
    save_to_current: bool,
) -> Result<()> {
    println!("{} {}", "Generating image:".bold(), prompt.italic());

    match client.generate_image(prompt).await {
        Ok(response) => {
            if response.success {
                if let Some(image_data) = response.image {
                    let output_path =
                        save_image(&image_data, output_dir, filename, save_to_current)?;
                    println!(
                        "{} Image saved to: {}",
                        "✓".green(),
                        output_path.display().to_string().bold()
                    );
                }
            } else {
                eprintln!(
                    "{} Generation failed: {}",
                    "✗".red(),
                    response.error.unwrap_or_else(|| "Unknown error".to_string())
                );
            }
        }
        Err(e) => eprintln!("{} Error: {}", "✗".red(), e),
    }

    Ok(())
}