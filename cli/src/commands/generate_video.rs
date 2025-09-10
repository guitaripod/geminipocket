use anyhow::Result;
use colored::*;
use std::path::Path;

use crate::api::GeminiClient;
use crate::utils::save_video;

pub async fn handle_generate_video(
    client: &GeminiClient,
    prompt: &str,
    output_dir: Option<&Path>,
    filename: Option<&str>,
    save_to_current: bool,
    negative_prompt: Option<&str>,
    aspect_ratio: Option<&str>,
    resolution: Option<&str>,
) -> Result<()> {
    println!("{} {}", "Generating video:".bold(), prompt.italic());

    match client.generate_video(prompt, negative_prompt, aspect_ratio, resolution).await {
        Ok(operation_name) => {
            println!("{} Started video generation (operation: {})", "✓".green(), operation_name);

            match client.poll_video_completion(&operation_name).await {
                Ok(video_uri) => {
                    println!("{} Video generation completed!", "✓".green());

                    // Download the video
                    let output_path = save_video(&video_uri, output_dir, filename, save_to_current).await?;
                    println!(
                        "{} Video saved to: {}",
                        "✓".green(),
                        output_path.display().to_string().bold()
                    );
                }
                Err(e) => {
                    eprintln!("{} Video generation failed: {}", "✗".red(), e);
                }
            }
        }
        Err(e) => {
            eprintln!("{} Error starting video generation: {}", "✗".red(), e);
        }
    }

    Ok(())
}