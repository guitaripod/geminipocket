pub mod openapi;
pub mod swagger_ui;

use worker::*;
use worker::d1::D1Type;
use serde::{Deserialize, Serialize};
use serde_json;


use crate::openapi::openapi_spec;
use crate::swagger_ui::swagger_ui_html;

#[derive(Deserialize)]
struct GenerateRequest {
    prompt: String,
}

#[derive(Deserialize)]
struct EditRequest {
    image: String,
    prompt: String,
    #[serde(default)]
    mime_type: String,
}

#[derive(Deserialize)]
struct GenerateVideoRequest {
    prompt: String,
    #[serde(default)]
    negative_prompt: Option<String>,
    #[serde(default)]
    aspect_ratio: Option<String>,
    #[serde(default)]
    resolution: Option<String>,
}

#[derive(Deserialize)]
struct EditVideoRequest {
    prompt: String,
    image: String,
    mime_type: String,
    #[serde(default)]
    negative_prompt: Option<String>,
    #[serde(default)]
    aspect_ratio: Option<String>,
    #[serde(default)]
    resolution: Option<String>,
}

#[derive(Serialize)]
struct VideoOperationResponse {
    success: bool,
    operation_name: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct VideoStatusResponse {
    #[serde(default)]
    done: bool,
    response: Option<VideoGenerationResponse>,
    name: Option<String>,
}

#[derive(Deserialize)]
struct VideoGenerationResponse {
    generate_video_response: GenerateVideoResponse,
}

#[derive(Deserialize)]
struct GenerateVideoResponse {
    generated_samples: Vec<VideoSample>,
}

#[derive(Deserialize)]
struct VideoSample {
    video: VideoFile,
}

#[derive(Deserialize)]
struct VideoFile {
    uri: String,
}

#[derive(Deserialize)]
struct GeminiError {
    error: GeminiErrorDetails,
}

#[derive(Deserialize)]
struct GeminiErrorDetails {
    code: u16,
    message: String,
    status: String,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: Option<GeminiContent>,
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum GeminiPart {
    Text {
        text: String,
    },
    Image {
        #[serde(rename = "inlineData")]
        inline_data: InlineData,
    },
}

#[derive(Debug, Deserialize)]
struct InlineData {
    #[serde(rename = "mimeType")]
    mime_type: String,
    data: String,
}


#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    success: bool,
    api_key: Option<String>,
    error: Option<String>,
}

#[derive(Serialize)]
struct User {
    id: String,
    email: String,
    api_key: String,
    created_at: String,
}

fn cors_headers() -> Headers {
    let headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "*").unwrap();
    headers.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS").unwrap();
    headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization").unwrap();
    headers
}

async fn call_gemini_generate(prompt: &str, api_key: &str) -> Result<GeminiResponse> {
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-image-preview:generateContent";

    let json_body = format!(
        r#"{{"contents":[{{"parts":[{{"text":"{}"}}]}}]}}"#,
        prompt.replace('"', "\\\"")
    );

    let headers = Headers::new();
    headers.set("Content-Type", "application/json").unwrap();
    headers.set("x-goog-api-key", api_key).unwrap();

    let request = Request::new_with_init(
        url,
        &RequestInit::new()
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(json_body.into())),
    )?;

    let mut response = Fetch::Request(request).send().await?;
    let text = response.text().await?;
    
    console_log!("Gemini API response length: {} bytes", text.len());
    
    let gemini_response: GeminiResponse = serde_json::from_str(&text)
        .map_err(|e| worker::Error::RustError(format!("Failed to parse Gemini response: {}", e)))?;
    
    Ok(gemini_response)
}

async fn call_gemini_edit(image_data: &str, prompt: &str, api_key: &str) -> Result<GeminiResponse> {
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-image-preview:generateContent";

    let json_body = format!(
        r#"{{"contents":[{{"parts":[{{"text":"{}"}},{{"inline_data":{{"mime_type":"image/png","data":"{}"}}}}]}}]}}"#,
        prompt.replace('"', "\\\""),
        image_data
    );

    let headers = Headers::new();
    headers.set("Content-Type", "application/json").unwrap();
    headers.set("x-goog-api-key", api_key).unwrap();

    let request = Request::new_with_init(
        url,
        &RequestInit::new()
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(json_body.into())),
    )?;

    let mut response = Fetch::Request(request).send().await?;
    let text = response.text().await?;
    
    console_log!("Gemini API response length: {} bytes", text.len());
    
    let gemini_response: GeminiResponse = serde_json::from_str(&text)
        .map_err(|e| worker::Error::RustError(format!("Failed to parse Gemini response: {}", e)))?;
    
    Ok(gemini_response)
}

fn extract_image_from_response(response: &GeminiResponse) -> Result<String> {
    if response.candidates.is_empty() {
        return Err(worker::Error::RustError("No candidates in Gemini response".into()));
    }

    for candidate in &response.candidates {
        if let Some(content) = &candidate.content {
            for part in &content.parts {
                if let GeminiPart::Image { inline_data } = part {
                    console_log!("Found image data, length: {}", inline_data.data.len());
                    return Ok(inline_data.data.clone());
                }
            }
        }
    }

    console_log!("No image data found in response");
    Err(worker::Error::RustError("No image data found in response".into()))
}

async fn call_veo_generate(prompt: &str, negative_prompt: Option<&str>, aspect_ratio: Option<&str>, resolution: Option<&str>, api_key: &str) -> Result<String> {
    let url = "https://generativelanguage.googleapis.com/v1beta/models/veo-3.0-fast-generate-001:predictLongRunning";

    console_log!("Starting Veo video generation with prompt: {}", prompt);
    console_log!("Negative prompt: {:?}", negative_prompt);
    console_log!("Aspect ratio: {:?}", aspect_ratio);
    console_log!("Resolution: {:?}", resolution);

    let mut instances = serde_json::json!({
        "prompt": prompt
    });

    if let Some(neg_prompt) = negative_prompt {
        instances["negativePrompt"] = serde_json::json!(neg_prompt);
    }

    let mut parameters = serde_json::json!({});
    if let Some(ar) = aspect_ratio {
        parameters["aspectRatio"] = serde_json::json!(ar);
    }
    if let Some(res) = resolution {
        parameters["resolution"] = serde_json::json!(res);
    }

    let json_body = serde_json::json!({
        "instances": [instances],
        "parameters": parameters
    });

    console_log!("Veo API request body: {}", json_body.to_string());

    let headers = Headers::new();
    headers.set("Content-Type", "application/json").unwrap();
    headers.set("x-goog-api-key", api_key).unwrap();

    let request = Request::new_with_init(
        url,
        &RequestInit::new()
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(json_body.to_string().into())),
    )?;

    console_log!("Sending request to Veo API...");
    let mut response = Fetch::Request(request).send().await?;
    console_log!("Veo API response status: {}", response.status_code());

    let text = response.text().await?;
    console_log!("Veo generate API response length: {} bytes", text.len());
    console_log!("Veo API response: {}", text);

    // Check if the response is an error
    if response.status_code() < 200 || response.status_code() >= 300 {
        // Try to parse as Gemini error response
        if let Ok(error_response) = serde_json::from_str::<GeminiError>(&text) {
            let error_msg = match error_response.error.code {
                429 => "Rate limit exceeded. You've reached your API quota. Please wait a few minutes before trying again.".to_string(),
                403 => "Access denied. Please check your API key and permissions.".to_string(),
                400 => format!("Invalid request: {}", error_response.error.message),
                500 => "Server error. Please try again later.".to_string(),
                _ => format!("API Error ({}): {}", error_response.error.code, error_response.error.message),
            };
            return Err(worker::Error::RustError(error_msg));
        } else {
            return Err(worker::Error::RustError(format!("API request failed with status {}: {}", response.status_code(), text)));
        }
    }

    let operation_response: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| worker::Error::RustError(format!("Failed to parse Veo response: {}", e)))?;

    let operation_name = operation_response["name"]
        .as_str()
        .ok_or_else(|| worker::Error::RustError("No operation name in response".into()))?;

    console_log!("Operation name: {}", operation_name);
    Ok(operation_name.to_string())
}

async fn call_veo_edit(image_data: &str, mime_type: &str, prompt: &str, negative_prompt: Option<&str>, aspect_ratio: Option<&str>, resolution: Option<&str>, api_key: &str) -> Result<String> {
    let url = "https://generativelanguage.googleapis.com/v1beta/models/veo-3.0-fast-generate-001:predictLongRunning";

    let mut instances = serde_json::json!({
        "prompt": prompt,
        "image": {
            "bytesBase64Encoded": image_data,
            "mimeType": mime_type
        }
    });

    if let Some(neg_prompt) = negative_prompt {
        instances["negativePrompt"] = serde_json::json!(neg_prompt);
    }

    let mut parameters = serde_json::json!({});
    if let Some(ar) = aspect_ratio {
        parameters["aspectRatio"] = serde_json::json!(ar);
    }
    if let Some(res) = resolution {
        parameters["resolution"] = serde_json::json!(res);
    }

    let json_body = serde_json::json!({
        "instances": [instances],
        "parameters": parameters
    });

    let headers = Headers::new();
    headers.set("Content-Type", "application/json").unwrap();
    headers.set("x-goog-api-key", api_key).unwrap();

    let request = Request::new_with_init(
        url,
        &RequestInit::new()
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(json_body.to_string().into())),
    )?;

    let mut response = Fetch::Request(request).send().await?;
    console_log!("Veo edit API response status: {}", response.status_code());

    let text = response.text().await?;
    console_log!("Veo edit API response length: {} bytes", text.len());
    console_log!("Veo edit API response: {}", text);

    // Check if the response is an error
    if response.status_code() < 200 || response.status_code() >= 300 {
        // Try to parse as Gemini error response
        if let Ok(error_response) = serde_json::from_str::<GeminiError>(&text) {
            let error_msg = match error_response.error.code {
                429 => "Rate limit exceeded. You've reached your API quota. Please wait a few minutes before trying again.".to_string(),
                403 => "Access denied. Please check your API key and permissions.".to_string(),
                400 => format!("Invalid request: {}", error_response.error.message),
                500 => "Server error. Please try again later.".to_string(),
                _ => format!("API Error ({}): {}", error_response.error.code, error_response.error.message),
            };
            return Err(worker::Error::RustError(error_msg));
        } else {
            return Err(worker::Error::RustError(format!("API request failed with status {}: {}", response.status_code(), text)));
        }
    }

    let operation_response: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| worker::Error::RustError(format!("Failed to parse Veo response: {}", e)))?;

    let operation_name = operation_response["name"]
        .as_str()
        .ok_or_else(|| worker::Error::RustError("No operation name in response".into()))?;

    Ok(operation_name.to_string())
}

async fn poll_video_operation(operation_name: &str, api_key: &str) -> Result<VideoStatusResponse> {
    let url = format!("https://generativelanguage.googleapis.com/v1beta/{}", operation_name);

    console_log!("Polling video operation: {}", operation_name);

    let headers = Headers::new();
    headers.set("x-goog-api-key", api_key).unwrap();

    let request = Request::new_with_init(
        &url,
        &RequestInit::new()
            .with_method(Method::Get)
            .with_headers(headers),
    )?;

    console_log!("Sending poll request to: {}", url);
    let mut response = Fetch::Request(request).send().await?;
    console_log!("Poll response status: {}", response.status_code());

    let text = response.text().await?;
    console_log!("Video operation poll response length: {} bytes", text.len());
    console_log!("Poll response body: {}", text);

    let status_response: VideoStatusResponse = serde_json::from_str(&text)
        .map_err(|e| worker::Error::RustError(format!("Failed to parse operation status: {}", e)))?;

    console_log!("Parsed status - done: {:?}", status_response.done);
    if status_response.done {
        console_log!("Video generation completed!");
    } else {
        console_log!("Video generation still in progress...");
    }

    Ok(status_response)
}

fn extract_video_uri(response: &VideoStatusResponse) -> Result<String> {
    console_log!("Extracting video URI from response...");
    console_log!("Response has response field: {}", response.response.is_some());

    if let Some(video_response) = &response.response {
        console_log!("Video response has generate_video_response field: {}", video_response.generate_video_response.generated_samples.len());
        if !video_response.generate_video_response.generated_samples.is_empty() {
            let video_uri = &video_response.generate_video_response.generated_samples[0].video.uri;
            console_log!("Found video URI: {}", video_uri);
            return Ok(video_uri.clone());
        } else {
            console_log!("generated_samples array is empty");
        }
    } else {
        console_log!("response.response is None");
    }

    console_log!("No video URI found in response");
    Err(worker::Error::RustError("No video URI found in response".into()))
}


fn generate_api_key() -> String {
    format!("gp_{}", uuid::Uuid::new_v4().to_string().replace("-", ""))
}

fn verify_password(password: &str, hash: &str) -> Result<bool> {
    bcrypt::verify(password, hash)
        .map_err(|e| worker::Error::RustError(format!("Password verification failed: {}", e)))
}

fn hash_password(password: &str) -> Result<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| worker::Error::RustError(format!("Password hashing failed: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_api_key() {
        let api_key = generate_api_key();
        assert!(api_key.starts_with("gp_"));
        assert_eq!(api_key.len(), 35); // "gp_" + 32 characters (UUID without hyphens)
    }

    #[test]
    fn test_hash_password() {
        let password = "test_password";
        let hashed = hash_password(password).unwrap();
        assert!(!hashed.is_empty());
        assert_ne!(hashed, password);
        assert!(verify_password(password, &hashed).unwrap());
        assert!(!verify_password("wrong_password", &hashed).unwrap());
    }

    #[test]
    fn test_validate_api_key_format() {
        let valid_key = "gp_12345678901234567890123456789012";
        let invalid_key = "invalid_key";

        assert!(valid_key.starts_with("gp_"));
        assert_eq!(valid_key.len(), 35);
        assert!(!invalid_key.starts_with("gp_") || invalid_key.len() != 35);
    }
}

async fn validate_api_key(env: &Env, api_key: &str) -> Result<bool> {
    if !api_key.starts_with("gp_") || api_key.len() != 35 {
        return Ok(false);
    }

    let db = env.d1("DB")?;
    let statement = db.prepare("SELECT COUNT(*) as count FROM users WHERE api_key = ?");
    let query = statement.bind_refs(&[D1Type::Text(api_key)])?;

    #[derive(serde::Deserialize)]
    struct CountResult {
        count: i32,
    }

    let result: Option<CountResult> = query.first(None).await?;
    Ok(result.map(|r| r.count > 0).unwrap_or(false))
}

async fn create_user(env: &Env, email: &str, password: &str) -> Result<String> {
    let db = env.d1("DB")?;
    let api_key = generate_api_key();
    let password_hash = hash_password(password)?;

    let check_statement = db.prepare("SELECT 1 FROM users WHERE email = ?");
    let check_query = check_statement.bind_refs(&[D1Type::Text(email)])?;
    let existing_user: Option<i32> = check_query.first(None).await?;

    if existing_user.is_some() {
        return Err(worker::Error::RustError("User already exists".into()));
    }

    let insert_statement = db.prepare(
        "INSERT INTO users (email, password_hash, api_key, created_at) VALUES (?, ?, ?, datetime('now'))"
    );
    let insert_query = insert_statement.bind_refs(&[
        D1Type::Text(email),
        D1Type::Text(&password_hash),
        D1Type::Text(&api_key),
    ])?;

    insert_query.run().await?;
    Ok(api_key)
}

async fn authenticate_user(env: &Env, email: &str, password: &str) -> Result<String> {
    let db = env.d1("DB")?;
    let statement = db.prepare("SELECT api_key, password_hash FROM users WHERE email = ?");
    let query = statement.bind_refs(&[D1Type::Text(email)])?;

    #[derive(serde::Deserialize)]
    struct UserCredentials {
        api_key: String,
        password_hash: String,
    }

    let result: Option<UserCredentials> = query.first(None).await?;

    match result {
        Some(credentials) => {
            if verify_password(password, &credentials.password_hash)? {
                Ok(credentials.api_key)
            } else {
                Err(worker::Error::RustError("Invalid credentials".into()))
            }
        }
        None => Err(worker::Error::RustError("Invalid credentials".into())),
    }
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    
    Router::new()
        .get("/", |_, _| {
            let headers = cors_headers();
            headers.set("Content-Type", "text/html").unwrap();
            Response::ok(include_str!("../../../web/public/index.html"))
                .map(|r| r.with_headers(headers))
        })
        .get("/styles.css", |_, _| {
            let headers = cors_headers();
            headers.set("Content-Type", "text/css").unwrap();
            Response::ok(include_str!("../../../web/public/styles.css"))
                .map(|r| r.with_headers(headers))
        })
        .get("/app.js", |_, _| {
            let headers = cors_headers();
            headers.set("Content-Type", "application/javascript").unwrap();
            Response::ok(include_str!("../../../web/public/app.js"))
                .map(|r| r.with_headers(headers))
        })
        .get("/health", |_, _| {
            Response::ok("{\"status\":\"healthy\"}")
        })
        .get("/openapi", |_, _| {
            Response::ok(openapi_spec().to_string())
                .map(|r| r.with_headers(cors_headers()))
        })
        .get("/docs", |_, _| {
            let headers = cors_headers();
            headers.set("Content-Type", "text/html").unwrap();
            Response::ok(swagger_ui_html())
                .map(|r| r.with_headers(headers))
        })
        .post_async("/register", |mut req, ctx| async move {
            let body = match req.json::<RegisterRequest>().await {
                Ok(body) => body,
                Err(_) => {
                    return Response::ok("{\"success\":false,\"error\":\"Invalid request body\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            match create_user(&ctx.env, &body.email, &body.password).await {
                Ok(api_key) => {
                    let response = AuthResponse {
                        success: true,
                        api_key: Some(api_key),
                        error: None,
                    };
                    Response::ok(serde_json::to_string(&response).unwrap())
                        .map(|r| r.with_headers(cors_headers()))
                }
                Err(e) => {
                    let response = AuthResponse {
                        success: false,
                        api_key: None,
                        error: Some(e.to_string()),
                    };
                    Response::ok(serde_json::to_string(&response).unwrap())
                        .map(|r| r.with_headers(cors_headers()))
                }
            }
        })
        .post_async("/login", |mut req, ctx| async move {
            let body = match req.json::<LoginRequest>().await {
                Ok(body) => body,
                Err(_) => {
                    return Response::ok("{\"success\":false,\"error\":\"Invalid request body\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            match authenticate_user(&ctx.env, &body.email, &body.password).await {
                Ok(api_key) => {
                    let response = AuthResponse {
                        success: true,
                        api_key: Some(api_key),
                        error: None,
                    };
                    Response::ok(serde_json::to_string(&response).unwrap())
                        .map(|r| r.with_headers(cors_headers()))
                }
                Err(e) => {
                    let response = AuthResponse {
                        success: false,
                        api_key: None,
                        error: Some(e.to_string()),
                    };
                    Response::ok(serde_json::to_string(&response).unwrap())
                        .map(|r| r.with_headers(cors_headers()))
                }
            }
        })
        .post_async("/generate", |mut req, ctx| async move {
            // Check API key from Authorization header
            let auth_header = match req.headers().get("Authorization") {
                Ok(Some(header)) => header,
                _ => {
                    return Response::ok("{\"success\":false,\"error\":\"Missing API key\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            let api_key = auth_header.trim_start_matches("Bearer ");
            if !validate_api_key(&ctx.env, api_key).await.unwrap_or(false) {
                return Response::ok("{\"success\":false,\"error\":\"Invalid API key\"}")
                    .map(|r| r.with_headers(cors_headers()));
            }

            let body = match req.json::<GenerateRequest>().await {
                Ok(body) => body,
                Err(_) => {
                    return Response::ok("{\"success\":false,\"error\":\"Invalid request body\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            let gemini_api_key = match ctx.env.secret("GEMINI_API_KEY") {
                Ok(key) => key.to_string(),
                Err(_) => {
                    return Response::ok("{\"success\":false,\"error\":\"API key not configured\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            match call_gemini_generate(&body.prompt, &gemini_api_key).await {
                Ok(gemini_response) => {
                    match extract_image_from_response(&gemini_response) {
                        Ok(image_data) => {
                            let response_json = format!(r#"{{"success":true,"image":"{}"}}"#, image_data);
                            Response::ok(response_json).map(|r| r.with_headers(cors_headers()))
                        }
                        Err(e) => {
                            console_log!("Failed to extract image: {}", e);
                            Response::ok(format!(r#"{{"success":false,"error":"{}"}}"#, e))
                                .map(|r| r.with_headers(cors_headers()))
                        }
                    }
                }
                Err(e) => {
                    console_log!("Failed to call Gemini API: {}", e);
                    Response::ok(format!(r#"{{"success":false,"error":"{}"}}"#, e))
                        .map(|r| r.with_headers(cors_headers()))
                }
            }
        })
        .post_async("/edit", |mut req, ctx| async move {
            // Check API key from Authorization header
            let auth_header = match req.headers().get("Authorization") {
                Ok(Some(header)) => header,
                _ => {
                    return Response::ok("{\"success\":false,\"error\":\"Missing API key\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            let api_key = auth_header.trim_start_matches("Bearer ");
            if !validate_api_key(&ctx.env, api_key).await.unwrap_or(false) {
                return Response::ok("{\"success\":false,\"error\":\"Invalid API key\"}")
                    .map(|r| r.with_headers(cors_headers()));
            }

            let body = match req.json::<EditRequest>().await {
                Ok(body) => body,
                Err(_) => {
                    return Response::ok("{\"success\":false,\"error\":\"Invalid request body\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            let gemini_api_key = match ctx.env.secret("GEMINI_API_KEY") {
                Ok(key) => key.to_string(),
                Err(_) => {
                    return Response::ok("{\"success\":false,\"error\":\"API key not configured\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            match call_gemini_edit(&body.image, &body.prompt, &gemini_api_key).await {
                Ok(gemini_response) => {
                    match extract_image_from_response(&gemini_response) {
                        Ok(image_data) => {
                            let response_json = format!(r#"{{"success":true,"image":"{}"}}"#, image_data);
                            Response::ok(response_json).map(|r| r.with_headers(cors_headers()))
                        }
                        Err(e) => {
                            console_log!("Failed to extract image: {}", e);
                            Response::ok(format!(r#"{{"success":false,"error":"{}"}}"#, e))
                                .map(|r| r.with_headers(cors_headers()))
                        }
                    }
                }
                Err(e) => {
                    console_log!("Failed to call Gemini API: {}", e);
                    Response::ok(format!(r#"{{"success":false,"error":"{}"}}"#, e))
                        .map(|r| r.with_headers(cors_headers()))
                }
            }
        })
        .post_async("/generate_video", |mut req, ctx| async move {
            // Check API key from Authorization header
            let auth_header = match req.headers().get("Authorization") {
                Ok(Some(header)) => header,
                _ => {
                    return Response::ok("{\"success\":false,\"error\":\"Missing API key\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            let api_key = auth_header.trim_start_matches("Bearer ");
            if !validate_api_key(&ctx.env, api_key).await.unwrap_or(false) {
                return Response::ok("{\"success\":false,\"error\":\"Invalid API key\"}")
                    .map(|r| r.with_headers(cors_headers()));
            }

            let body = match req.json::<GenerateVideoRequest>().await {
                Ok(body) => body,
                Err(_) => {
                    return Response::ok("{\"success\":false,\"error\":\"Invalid request body\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            let gemini_api_key = match ctx.env.secret("GEMINI_API_KEY") {
                Ok(key) => key.to_string(),
                Err(_) => {
                    return Response::ok("{\"success\":false,\"error\":\"API key not configured\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            match call_veo_generate(
                &body.prompt,
                body.negative_prompt.as_deref(),
                body.aspect_ratio.as_deref(),
                body.resolution.as_deref(),
                &gemini_api_key
            ).await {
                Ok(operation_name) => {
                    let response = VideoOperationResponse {
                        success: true,
                        operation_name: Some(operation_name),
                        error: None,
                    };
                    Response::ok(serde_json::to_string(&response).unwrap())
                        .map(|r| r.with_headers(cors_headers()))
                }
                Err(e) => {
                    console_log!("Failed to start video generation: {}", e);
                    let response = VideoOperationResponse {
                        success: false,
                        operation_name: None,
                        error: Some(e.to_string()),
                    };
                    Response::ok(serde_json::to_string(&response).unwrap())
                        .map(|r| r.with_headers(cors_headers()))
                }
            }
        })
        .post_async("/edit_video", |mut req, ctx| async move {
            // Check API key from Authorization header
            let auth_header = match req.headers().get("Authorization") {
                Ok(Some(header)) => header,
                _ => {
                    return Response::ok("{\"success\":false,\"error\":\"Missing API key\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            let api_key = auth_header.trim_start_matches("Bearer ");
            if !validate_api_key(&ctx.env, api_key).await.unwrap_or(false) {
                return Response::ok("{\"success\":false,\"error\":\"Invalid API key\"}")
                    .map(|r| r.with_headers(cors_headers()));
            }

            let body = match req.json::<EditVideoRequest>().await {
                Ok(body) => body,
                Err(_) => {
                    return Response::ok("{\"success\":false,\"error\":\"Invalid request body\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            let gemini_api_key = match ctx.env.secret("GEMINI_API_KEY") {
                Ok(key) => key.to_string(),
                Err(_) => {
                    return Response::ok("{\"success\":false,\"error\":\"API key not configured\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            match call_veo_edit(
                &body.image,
                &body.mime_type,
                &body.prompt,
                body.negative_prompt.as_deref(),
                body.aspect_ratio.as_deref(),
                body.resolution.as_deref(),
                &gemini_api_key
            ).await {
                Ok(operation_name) => {
                    let response = VideoOperationResponse {
                        success: true,
                        operation_name: Some(operation_name),
                        error: None,
                    };
                    Response::ok(serde_json::to_string(&response).unwrap())
                        .map(|r| r.with_headers(cors_headers()))
                }
                Err(e) => {
                    console_log!("Failed to start video editing: {}", e);
                    let response = VideoOperationResponse {
                        success: false,
                        operation_name: None,
                        error: Some(e.to_string()),
                    };
                    Response::ok(serde_json::to_string(&response).unwrap())
                        .map(|r| r.with_headers(cors_headers()))
                }
            }
        })
        .get_async("/video_status/*operation", |req, ctx| async move {
            console_log!("=== VIDEO STATUS ENDPOINT CALLED ===");

            // Check API key from Authorization header
            let auth_header = match req.headers().get("Authorization") {
                Ok(Some(header)) => {
                    console_log!("✅ Found Authorization header: {}", header);
                    header
                },
                _ => {
                    console_log!("❌ Missing Authorization header");
                    return Response::ok("{\"success\":false,\"error\":\"Missing API key\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            let api_key = auth_header.trim_start_matches("Bearer ");
            console_log!("🔑 Extracted API key: {}...", &api_key[..8]);

            let is_valid = validate_api_key(&ctx.env, api_key).await.unwrap_or(false);
            console_log!("🔐 API key validation result: {}", is_valid);

            if !is_valid {
                console_log!("❌ API key validation failed - returning error");
                return Response::ok("{\"success\":false,\"error\":\"Invalid API key\"}")
                    .map(|r| r.with_headers(cors_headers()));
            }

            let operation_name = match ctx.param("operation") {
                Some(name) => {
                    // Remove leading slash from wildcard parameter
                    let clean_name = name.trim_start_matches('/');
                    console_log!("📝 Operation name from URL: {}", clean_name);
                    clean_name.to_string()
                },
                None => {
                    console_log!("❌ Missing operation name parameter");
                    return Response::ok("{\"success\":false,\"error\":\"Missing operation name\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            let gemini_api_key = match ctx.env.secret("GEMINI_API_KEY") {
                Ok(key) => key.to_string(),
                Err(_) => {
                    return Response::ok("{\"success\":false,\"error\":\"API key not configured\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            console_log!("Video status requested for operation: {}", operation_name);

            match poll_video_operation(&operation_name, &gemini_api_key).await {
                Ok(status) => {
                    console_log!("Poll successful, status.done: {:?}", status.done);
                    console_log!("Status has response: {}", status.response.is_some());

                    if status.done && status.response.is_some() {
                        console_log!("Video is done, extracting URI...");
                        match extract_video_uri(&status) {
                            Ok(video_uri) => {
                                console_log!("Video URI extracted: {}", video_uri);
                                let response_json = format!(r#"{{"success":true,"done":true,"video_uri":"{}"}}"#, video_uri);
                                console_log!("Returning success response with video URI");
                                Response::ok(response_json).map(|r| r.with_headers(cors_headers()))
                            }
                            Err(e) => {
                                console_log!("Failed to extract video URI: {}", e);
                                Response::ok(format!(r#"{{"success":false,"done":true,"error":"{}"}}"#, e))
                                    .map(|r| r.with_headers(cors_headers()))
                            }
                        }
                    } else {
                        console_log!("Video still in progress, returning done:false");
                        Response::ok(r#"{"success":true,"done":false}"#)
                            .map(|r| r.with_headers(cors_headers()))
                    }
                }
                Err(e) => {
                    console_log!("Failed to poll video operation: {}", e);
                    Response::ok(format!(r#"{{"success":false,"error":"{}"}}"#, e))
                        .map(|r| r.with_headers(cors_headers()))
                }
            }
        })
        .options("/*catchall", |_, _| {
            Response::ok("").map(|r| r.with_headers(cors_headers()))
        })
        .run(req, env)
        .await
}