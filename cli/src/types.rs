use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct GenerateRequest {
    pub prompt: String,
}

#[derive(Serialize, Deserialize)]
pub struct EditRequest {
    pub prompt: String,
    pub image: String,
    pub mime_type: String,
}

#[derive(Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub image: Option<String>,
    #[allow(dead_code)]
    pub mime_type: Option<String>,
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct ApiInfo {
    pub name: String,
    pub version: String,
    pub endpoints: serde_json::Value,
}

#[derive(Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: Option<f64>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub api_url: Option<String>,
    pub output_dir: Option<PathBuf>,
}