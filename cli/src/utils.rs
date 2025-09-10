use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::Client;
use std::fs;
use std::path::{Path, PathBuf};

pub fn save_image(
    base64_data: &str,
    output_dir: Option<&Path>,
    filename: Option<&str>,
    save_to_current: bool,
) -> Result<PathBuf> {
    let image_data = STANDARD.decode(base64_data)?;

    let output_dir = if save_to_current {
        PathBuf::from(".")
    } else if let Some(dir) = output_dir {
        dir.to_path_buf()
    } else {
        dirs::picture_dir().unwrap_or_else(|| PathBuf::from("."))
    };

    fs::create_dir_all(&output_dir)?;

    let filename = filename.unwrap_or("gemini_image");
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let final_filename = format!("{}_{}.png", filename, timestamp);
    let output_path = output_dir.join(final_filename);

    fs::write(&output_path, image_data)?;
    Ok(output_path)
}

pub async fn save_video(
    video_uri: &str,
    output_dir: Option<&Path>,
    filename: Option<&str>,
    save_to_current: bool,
) -> Result<PathBuf> {
    let client = Client::new();

    let output_dir = if save_to_current {
        PathBuf::from(".")
    } else if let Some(dir) = output_dir {
        dir.to_path_buf()
    } else {
        dirs::video_dir().unwrap_or_else(|| dirs::picture_dir().unwrap_or_else(|| PathBuf::from(".")))
    };

    fs::create_dir_all(&output_dir)?;

    let filename = filename.unwrap_or("gemini_video");
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let final_filename = format!("{}_{}.mp4", filename, timestamp);
    let output_path = output_dir.join(final_filename);

    let response = client.get(video_uri).send().await?;
    let video_data = response.bytes().await?;

    fs::write(&output_path, video_data)?;
    Ok(output_path)
}

pub fn detect_mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        _ => "image/jpeg",
    }
}