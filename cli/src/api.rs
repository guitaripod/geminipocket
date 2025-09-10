use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::path::Path;
use std::time::Duration;

use crate::types::{ApiInfo, ApiResponse, AuthResponse, EditRequest, EditVideoRequest, GenerateRequest, GenerateVideoRequest, HealthResponse, LoginRequest, RegisterRequest, VideoOperationResponse, VideoStatusResponse};

pub struct GeminiClient {
    client: Client,
    api_url: String,
    api_key: Option<String>,
}

impl GeminiClient {
    pub fn new(api_url: String) -> Self {
        Self {
            client: Client::new(),
            api_url,
            api_key: None,
        }
    }

    pub fn with_api_key(api_url: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_url,
            api_key: Some(api_key),
        }
    }

    pub async fn register(&self, email: &str, password: &str) -> Result<AuthResponse> {
        let response = self
            .client
            .post(format!("{}/register", self.api_url))
            .json(&RegisterRequest {
                email: email.to_string(),
                password: password.to_string(),
            })
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(anyhow::anyhow!(
                "Registration failed with status: {}",
                response.status()
            ))
        }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<AuthResponse> {
        let response = self
            .client
            .post(format!("{}/login", self.api_url))
            .json(&LoginRequest {
                email: email.to_string(),
                password: password.to_string(),
            })
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(anyhow::anyhow!(
                "Login failed with status: {}",
                response.status()
            ))
        }
    }

    pub async fn generate_image(&self, prompt: &str) -> Result<ApiResponse> {
        let spinner = create_spinner("Generating image...");

        let mut request = self
            .client
            .post(format!("{}/generate", self.api_url))
            .json(&GenerateRequest {
                prompt: prompt.to_string(),
            });

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request.send().await?;

        spinner.finish_and_clear();

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(anyhow::anyhow!(
                "API request failed with status: {}",
                response.status()
            ))
        }
    }

    pub async fn edit_image(&self, image_path: &Path, prompt: &str) -> Result<ApiResponse> {
        let spinner = create_spinner("Reading image...");

        let image_data = std::fs::read(image_path)?;
        let image_base64 = STANDARD.encode(&image_data);

        let mime_type = match image_path.extension().and_then(|s| s.to_str()) {
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("webp") => "image/webp",
            _ => "image/jpeg",
        }
        .to_string();

        spinner.set_message("Editing image...");

        let mut request = self
            .client
            .post(format!("{}/edit", self.api_url))
            .json(&EditRequest {
                prompt: prompt.to_string(),
                image: image_base64,
                mime_type,
            });

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request.send().await?;

        spinner.finish_and_clear();

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(anyhow::anyhow!(
                "API request failed with status: {}",
                response.status()
            ))
        }
    }

    pub async fn health(&self) -> Result<HealthResponse> {
        let response = self
            .client
            .get(format!("{}/health", self.api_url))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(anyhow::anyhow!("Health check failed"))
        }
    }

    pub async fn info(&self) -> Result<ApiInfo> {
        let response = self.client.get(&self.api_url).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(anyhow::anyhow!("Failed to get API info"))
        }
    }

    pub async fn generate_video(
        &self,
        prompt: &str,
        negative_prompt: Option<&str>,
        aspect_ratio: Option<&str>,
        resolution: Option<&str>,
    ) -> Result<String> {
        let spinner = create_spinner("Starting video generation...");

        let request_body = GenerateVideoRequest {
            prompt: prompt.to_string(),
            negative_prompt: negative_prompt.map(|s| s.to_string()),
            aspect_ratio: aspect_ratio.map(|s| s.to_string()),
            resolution: resolution.map(|s| s.to_string()),
        };

        let mut request = self
            .client
            .post(format!("{}/generate_video", self.api_url))
            .json(&request_body);

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request.send().await?;
        spinner.finish_and_clear();

        if response.status().is_success() {
            let operation_response: VideoOperationResponse = response.json().await?;
            if operation_response.success {
                if let Some(operation_name) = operation_response.operation_name {
                    Ok(operation_name)
                } else {
                    Err(anyhow::anyhow!("No operation name in response"))
                }
            } else {
                Err(anyhow::anyhow!(
                    "Video generation failed: {}",
                    operation_response.error.unwrap_or_else(|| "Unknown error".to_string())
                ))
            }
        } else {
            Err(anyhow::anyhow!(
                "API request failed with status: {}",
                response.status()
            ))
        }
    }

    pub async fn edit_video(
        &self,
        image_path: &Path,
        prompt: &str,
        negative_prompt: Option<&str>,
        aspect_ratio: Option<&str>,
        resolution: Option<&str>,
    ) -> Result<String> {
        let spinner = create_spinner("Reading image for video editing...");

        let image_data = std::fs::read(image_path)?;
        let image_base64 = STANDARD.encode(&image_data);

        let mime_type = match image_path.extension().and_then(|s| s.to_str()) {
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("webp") => "image/webp",
            _ => "image/jpeg",
        }
        .to_string();

        spinner.set_message("Starting video editing...");

        let request_body = EditVideoRequest {
            prompt: prompt.to_string(),
            image: image_base64,
            mime_type,
            negative_prompt: negative_prompt.map(|s| s.to_string()),
            aspect_ratio: aspect_ratio.map(|s| s.to_string()),
            resolution: resolution.map(|s| s.to_string()),
        };

        let mut request = self
            .client
            .post(format!("{}/edit_video", self.api_url))
            .json(&request_body);

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request.send().await?;
        spinner.finish_and_clear();

        if response.status().is_success() {
            let operation_response: VideoOperationResponse = response.json().await?;
            if operation_response.success {
                if let Some(operation_name) = operation_response.operation_name {
                    Ok(operation_name)
                } else {
                    Err(anyhow::anyhow!("No operation name in response"))
                }
            } else {
                Err(anyhow::anyhow!(
                    "Video editing failed: {}",
                    operation_response.error.unwrap_or_else(|| "Unknown error".to_string())
                ))
            }
        } else {
            Err(anyhow::anyhow!(
                "API request failed with status: {}",
                response.status()
            ))
        }
    }

    pub async fn check_video_status(&self, operation_name: &str) -> Result<VideoStatusResponse> {
        let mut request = self
            .client
            .get(format!("{}/video_status/{}", self.api_url, operation_name));

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request.send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(anyhow::anyhow!(
                "Status check failed with status: {}",
                response.status()
            ))
        }
    }

    pub async fn poll_video_completion(&self, operation_name: &str) -> Result<String> {
        let spinner = create_spinner("Generating video...");

        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

            match self.check_video_status(operation_name).await {
                Ok(status) => {
                    if status.success {
                        if let Some(true) = status.done {
                            if let Some(video_uri) = status.video_uri {
                                spinner.finish_and_clear();
                                return Ok(video_uri);
                            } else {
                                spinner.finish_and_clear();
                                return Err(anyhow::anyhow!("Video generation completed but no URI provided"));
                            }
                        }
                        // Still processing, continue polling
                        spinner.set_message("Still generating video...");
                    } else {
                        spinner.finish_and_clear();
                        return Err(anyhow::anyhow!(
                            "Video generation failed: {}",
                            status.error.unwrap_or_else(|| "Unknown error".to_string())
                        ));
                    }
                }
                Err(e) => {
                    spinner.finish_and_clear();
                    return Err(anyhow::anyhow!("Failed to check video status: {}", e));
                }
            }
        }
    }
}

fn create_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner
}