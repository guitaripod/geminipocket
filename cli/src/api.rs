use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::path::Path;
use std::time::Duration;

use crate::types::{ApiInfo, ApiResponse, AuthResponse, EditRequest, GenerateRequest, HealthResponse, LoginRequest, RegisterRequest};

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