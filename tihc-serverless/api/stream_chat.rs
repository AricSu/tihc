use axum::http::Uri;
use axum::response::{Html, IntoResponse};
use axum::{
    Router,
    routing::{get, post},
};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use vercel_runtime::Error;
use vercel_runtime::axum::VercelLayer;

async fn home() -> impl IntoResponse {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>tihc</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            background-color: #000000;
            color: #ffffff;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Inter', sans-serif;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: 2rem;
        }
        .container {
            width: 100%;
            max-width: 600px;
        }
        h1 {
            font-size: 2.25rem;
            font-weight: 500;
            margin-bottom: 2rem;
            text-align: left;
            letter-spacing: -0.025em;
        }
        button {
            background-color: #171717;
            color: #ffffff;
            border: 1px solid #333333;
            padding: 8px 16px;
            font-size: 0.875rem;
            font-weight: 500;
            border-radius: 4px;
            cursor: pointer;
            transition: all 0.15s ease;
            margin-bottom: 1.5rem;
            font-family: inherit;
        }
        button:hover {
            background-color: #262626;
            border-color: #404040;
        }
        button:disabled {
            background-color: #0a0a0a;
            color: #666666;
            border-color: #262626;
            cursor: not-allowed;
        }
        #stream-container {
            background-color: #0a0a0a;
            border: 1px solid #262626;
            border-radius: 4px;
            padding: 1rem;
            margin-top: 1rem;
            min-height: 200px;
            display: block;
        }
        #stream-content {
            white-space: pre-wrap;
            font-family: 'SF Mono', 'Monaco', 'Consolas', monospace;
            font-size: 0.8rem;
            line-height: 1.5;
            color: #e5e5e5;
        }
        .loading {
            color: #888888;
            font-style: italic;
        }
    </style>
</head>
<body>
    Welcome to tihc serverless API powered by AskAric!
</body>
</html>"#;

    Html(html)
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    format!("Axum fallback for path {}", uri.path())
}

use axum::extract::Json;
use axum::http::{HeaderMap, StatusCode};
use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE};
use futures_util::StreamExt;
use hyper::body::Bytes;
use log::{debug, info, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use vercel_runtime::axum::stream_response;

#[derive(Deserialize, Serialize, Debug)]
pub struct ChatRequest {
    pub messages: Vec<Message>,
    pub chat_engine: String,
    pub stream: bool,
    pub chat_id: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
struct GoogleTokenInfo {
    aud: Option<String>,
    email: Option<String>,
    hd: Option<String>,
    sub: Option<String>,
    expires_in: Option<String>,
}

fn env_trimmed(key: &str) -> String {
    env::var(key).unwrap_or_default().trim().to_string()
}

fn auth_required() -> bool {
    let raw = env_trimmed("REQUIRE_AUTH");
    if raw.is_empty() {
        return !env_trimmed("GOOGLE_CLIENT_ID").is_empty();
    }
    matches!(raw.to_lowercase().as_str(), "1" | "true" | "yes" | "y")
}

fn get_bearer_token(headers: &HeaderMap) -> Option<String> {
    let value = headers.get("authorization")?.to_str().ok()?.trim().to_string();
    let lower = value.to_lowercase();
    if lower.starts_with("bearer ") {
        Some(value[7..].trim().to_string())
    } else {
        None
    }
}

async fn verify_google_token(client: &Client, token: &str) -> Result<GoogleTokenInfo, String> {
    let is_jwt = token.split('.').count() == 3;
    let req = client.get("https://oauth2.googleapis.com/tokeninfo");
    let req = if is_jwt {
        req.query(&[("id_token", token)])
    } else {
        req.query(&[("access_token", token)])
    };

    let res = req.send().await.map_err(|e| format!("tokeninfo request failed: {e}"))?;
    let status = res.status();
    if !status.is_success() {
        return Err(format!("tokeninfo rejected: http {status}"));
    }
    res.json::<GoogleTokenInfo>()
        .await
        .map_err(|e| format!("tokeninfo parse failed: {e}"))
}

fn enforce_audience(info: &GoogleTokenInfo) -> Result<(), String> {
    let expected_aud = env_trimmed("GOOGLE_CLIENT_ID");
    if expected_aud.is_empty() {
        return Ok(());
    }
    let aud = info.aud.clone().unwrap_or_default();
    if aud != expected_aud {
        return Err("google token audience mismatch".to_string());
    }
    Ok(())
}

fn enforce_workspace_domain(info: &GoogleTokenInfo) -> Result<(), String> {
    let expected_domain = env_trimmed("GOOGLE_WORKSPACE_DOMAIN");
    if expected_domain.is_empty() {
        return Ok(());
    }

    if let Some(hd) = &info.hd {
        if hd.trim().eq_ignore_ascii_case(&expected_domain) {
            return Ok(());
        }
    }

    let email = info.email.clone().unwrap_or_default();
    let expected_suffix = format!("@{}", expected_domain.to_lowercase());
    if email.to_lowercase().ends_with(&expected_suffix) {
        return Ok(());
    }

    Err("google workspace domain not allowed".to_string())
}

#[derive(Serialize)]
struct OpenAiRequest<'a> {
    model: &'a str,
    messages: &'a [Message],
    stream: bool,
}

fn openai_model() -> String {
  let model = env_trimmed("OPENAI_MODEL");
  if model.is_empty() {
    "gpt-4o-mini".to_string()
  } else {
    model
  }
}

fn rag_max_chars() -> usize {
    let raw = env_trimmed("RAG_MAX_CHARS");
    raw.parse::<usize>().ok().filter(|v| *v > 0).unwrap_or(20000)
}

async fn fetch_tidb_rag_context(client: &Client, payload: &ChatRequest) -> Result<String, String> {
    let api_url = env_trimmed("TIDB_API_URL");
    let token = env_trimmed("TIDB_API_TOKEN");
    if api_url.is_empty() || token.is_empty() {
        return Err("Missing TIDB_API_URL or TIDB_API_TOKEN".to_string());
    }

    let req_payload = ChatRequest {
        messages: payload.messages.clone(),
        chat_engine: "tidb".to_string(),
        stream: false,
        chat_id: payload.chat_id.clone(),
    };

    let res = client
        .post(&api_url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .json(&req_payload)
        .send()
        .await
        .map_err(|e| format!("TiDB RAG request error: {e}"))?;

    let status = res.status();
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(format!("TiDB RAG http {status}: {body}"));
    }

    let text = res.text().await.map_err(|e| format!("TiDB RAG read error: {e}"))?;
    let max_chars = rag_max_chars();
    if text.len() > max_chars {
        Ok(text[..max_chars].to_string())
    } else {
        Ok(text)
    }
}

fn build_openai_messages_with_rag(payload: &ChatRequest, rag_context: &str) -> Vec<Message> {
    let mut messages = Vec::with_capacity(payload.messages.len() + 1);
    let system = Message {
        role: "system".to_string(),
        content: format!(
            "You have access to TiDB documentation context retrieved by GraphRAG.\n\
Use it to answer accurately, but do not fabricate details beyond it.\n\n\
TiDB Docs Context:\n{}",
            rag_context
        ),
    };
    messages.push(system);
    messages.extend(payload.messages.iter().cloned());
    messages
}

async fn stream_openai_to_text(
    client: &Client,
    payload: &ChatRequest,
    tx: tokio::sync::mpsc::Sender<Result<Bytes, std::io::Error>>,
) {
    let api_key = env_trimmed("OPENAI_API_KEY");
    if api_key.is_empty() {
        let _ = tx
            .send(Ok(Bytes::from(
                "Missing OPENAI_API_KEY on serverless.\n",
            )))
            .await;
        return;
    }

    let req = OpenAiRequest {
        model: &openai_model(),
        messages: &payload.messages,
        stream: true,
    };

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("Accept", "text/event-stream")
        .json(&req)
        .send()
        .await;

    let response = match res {
        Ok(r) => r,
        Err(e) => {
            let _ = tx
                .send(Ok(Bytes::from(format!(
                    "OpenAI request error: {e}\n"
                ))))
                .await;
            return;
        }
    };

    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    while let Some(item) = stream.next().await {
        match item {
            Ok(chunk) => {
                let text = match std::str::from_utf8(&chunk) {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                buffer.push_str(text);

                while let Some(pos) = buffer.find('\n') {
                    let line = buffer[..pos].trim().to_string();
                    buffer.drain(..=pos);

                    if line.is_empty() {
                        continue;
                    }
                    if !line.starts_with("data:") {
                        continue;
                    }
                    let data = line.trim_start_matches("data:").trim();
                    if data == "[DONE]" {
                        return;
                    }
                    let parsed: serde_json::Value = match serde_json::from_str(data) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                    let delta = parsed["choices"][0]["delta"]["content"]
                        .as_str()
                        .or_else(|| parsed["choices"][0]["message"]["content"].as_str());
                    if let Some(token) = delta {
                        if tx.send(Ok(Bytes::from(token.to_string()))).await.is_err() {
                            return;
                        }
                    }
                }
            }
            Err(e) => {
                let _ = tx
                    .send(Ok(Bytes::from(format!(
                        "OpenAI stream error: {e}\n"
                    ))))
                    .await;
                return;
            }
        }
    }
}

async fn proxy_stream(
    client: &Client,
    api_url: String,
    token: String,
    payload: &ChatRequest,
    tx: tokio::sync::mpsc::Sender<Result<Bytes, std::io::Error>>,
) {
    let res = client
        .post(&api_url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .json(payload)
        .send()
        .await;

    match res {
        Ok(response) => {
            let mut stream = response.bytes_stream();
            while let Some(item) = stream.next().await {
                match item {
                    Ok(chunk) => {
                        if tx.send(Ok(Bytes::from(chunk))).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Ok(Bytes::from(format!("Error: {e}\n")))).await;
                        break;
                    }
                }
            }
        }
        Err(e) => {
            let _ = tx
                .send(Ok(Bytes::from(format!(
                    "Request error: {e}\n"
                ))))
                .await;
        }
    }
}

pub async fn stream_chat(headers: HeaderMap, Json(payload): Json<ChatRequest>) -> axum::response::Response {
    info!(
        "[stream_chat] request: engine={} stream={} messages={} chat_id={:?}",
        payload.chat_engine,
        payload.stream,
        payload.messages.len(),
        payload.chat_id
    );
    let client = Client::new();

    if auth_required() {
        let Some(token) = get_bearer_token(&headers) else {
            return (StatusCode::UNAUTHORIZED, "Missing Authorization: Bearer <google_token>\n")
                .into_response();
        };
        match verify_google_token(&client, &token).await.and_then(|info| {
            enforce_audience(&info)?;
            enforce_workspace_domain(&info)?;
            Ok(info)
        }) {
            Ok(info) => {
                debug!(
                    "[stream_chat] auth ok: sub={:?} hd={:?} expires_in={:?}",
                    info.sub, info.hd, info.expires_in
                );
            }
            Err(e) => {
                warn!("[stream_chat] auth failed: {}", e);
                return (StatusCode::UNAUTHORIZED, format!("Unauthorized: {e}\n")).into_response();
            }
        }
    }

    if let Ok(payload_json) = serde_json::to_string(&payload) {
        debug!("[stream_chat] serialized payload: {}", payload_json);
    }
    let mut response = stream_response(|tx| async move {
        let engine = payload.chat_engine.trim().to_lowercase();
        if engine == "openai" {
            info!("[stream_chat] engine=openai");
            stream_openai_to_text(&client, &payload, tx).await;
            return;
        }
        if engine == "openai_rag" || engine == "openai+rag" {
            info!("[stream_chat] engine=openai_rag");
            match fetch_tidb_rag_context(&client, &payload).await {
                Ok(context) => {
                    let messages = build_openai_messages_with_rag(&payload, &context);
                    let rag_payload = ChatRequest {
                        messages,
                        chat_engine: "openai".to_string(),
                        stream: true,
                        chat_id: payload.chat_id.clone(),
                    };
                    stream_openai_to_text(&client, &rag_payload, tx).await;
                }
                Err(e) => {
                    let _ = tx
                        .send(Ok(Bytes::from(format!(
                            "[warn] doc RAG unavailable, continuing without it: {e}\n\n"
                        ))))
                        .await;
                    stream_openai_to_text(&client, &payload, tx).await;
                }
            }
            return;
        }

        if engine == "manus" {
            let api_url = env_trimmed("MANUS_API_URL");
            let token = env_trimmed("MANUS_API_TOKEN");
            if api_url.is_empty() || token.is_empty() {
                let _ = tx
                    .send(Ok(Bytes::from(
                        "Missing MANUS_API_URL or MANUS_API_TOKEN on serverless.\n",
                    )))
                    .await;
                return;
            }
            info!("[stream_chat] engine=manus proxy={}", api_url);
            proxy_stream(&client, api_url, token, &payload, tx).await;
            return;
        }

        let api_url = env_trimmed("TIDB_API_URL");
        let token = env_trimmed("TIDB_API_TOKEN");
        if api_url.is_empty() || token.is_empty() {
            let _ = tx
                .send(Ok(Bytes::from(
                    "Missing TIDB_API_URL or TIDB_API_TOKEN on serverless.\n",
                )))
                .await;
            return;
        }
        info!("[stream_chat] engine=tidb proxy={}", api_url);
        proxy_stream(&client, api_url, token, &payload, tx).await;
    })
    .into_response();

    response.headers_mut().insert(CONTENT_TYPE, "text/plain; charset=utf-8".parse().unwrap());
    response.headers_mut().insert(CACHE_CONTROL, "no-store".parse().unwrap());
    response
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, &log_level);
    env_logger::Builder::from_env(env).init();
    let router = Router::new()
        .route("/", get(home))
        .route("/api/stream_chat", post(stream_chat))
        .fallback(fallback);

    let app = ServiceBuilder::new()
        .layer(VercelLayer::new())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .service(router);
    vercel_runtime::run(app).await
}
