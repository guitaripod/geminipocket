pub mod openapi;
pub mod swagger_ui;

use worker::*;
use serde::Deserialize;

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

fn cors_headers() -> Headers {
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "*").unwrap();
    headers.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS").unwrap();
    headers.set("Access-Control-Allow-Headers", "Content-Type").unwrap();
    headers
}

async fn call_gemini_generate(prompt: &str, api_key: &str) -> Result<GeminiResponse> {
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-image-preview:generateContent";

    let json_body = format!(
        r#"{{"contents":[{{"parts":[{{"text":"{}"}}]}}]}}"#,
        prompt.replace('"', "\\\"")
    );

    let mut headers = Headers::new();
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

    let mut headers = Headers::new();
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

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    
    Router::new()
        .get("/", |_, _| {
            let mut headers = cors_headers();
            headers.set("Content-Type", "text/html").unwrap();
            Response::ok(swagger_ui_html())
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
            let mut headers = cors_headers();
            headers.set("Content-Type", "text/html").unwrap();
            Response::ok(swagger_ui_html())
                .map(|r| r.with_headers(headers))
        })
        .post_async("/generate", |mut req, env| async move {
            let body = match req.json::<GenerateRequest>().await {
                Ok(body) => body,
                Err(_) => {
                    return Response::ok("{\"success\":false,\"error\":\"Invalid request body\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            let api_key = match env.secret("GEMINI_API_KEY") {
                Ok(key) => key.to_string(),
                Err(_) => {
                    return Response::ok("{\"success\":false,\"error\":\"API key not configured\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            match call_gemini_generate(&body.prompt, &api_key).await {
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
        .post_async("/edit", |mut req, env| async move {
            let body = match req.json::<EditRequest>().await {
                Ok(body) => body,
                Err(_) => {
                    return Response::ok("{\"success\":false,\"error\":\"Invalid request body\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            let api_key = match env.secret("GEMINI_API_KEY") {
                Ok(key) => key.to_string(),
                Err(_) => {
                    return Response::ok("{\"success\":false,\"error\":\"API key not configured\"}")
                        .map(|r| r.with_headers(cors_headers()));
                }
            };

            match call_gemini_edit(&body.image, &body.prompt, &api_key).await {
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
        .options("/*catchall", |_, _| {
            Response::ok("").map(|r| r.with_headers(cors_headers()))
        })
        .run(req, env)
        .await
}