use axum::extract::Json;
use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE};
use axum::http::{HeaderMap, StatusCode, Uri};
use axum::response::{Html, IntoResponse};
use axum::{
    Router,
    routing::{get, post},
};
use futures_util::StreamExt;
use hyper::body::Bytes;
use log::{info, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use vercel_runtime::Error;
use vercel_runtime::axum::{VercelLayer, stream_response};

#[derive(Deserialize, Serialize, Debug, Clone)]
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
}

fn env_trimmed(key: &str) -> String {
    env::var(key).unwrap_or_default().trim().to_string()
}

fn auth_required() -> bool {
    let raw = env_trimmed("REQUIRE_AUTH");
    if raw.is_empty() {
        return false;
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

    let res = req
        .send()
        .await
        .map_err(|e| format!("tokeninfo request failed: {e}"))?;
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

async fn proxy_stream(
    client: &Client,
    api_url: String,
    token: String,
    payload: &ChatRequest,
    tx: tokio::sync::mpsc::Sender<Result<Bytes, std::io::Error>>,
) {
    let res = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header("Accept", "text/plain, application/json")
        .header("Authorization", format!("Bearer {}", token))
        .json(payload)
        .send()
        .await;

    let res = match res {
        Ok(r) => r,
        Err(e) => {
            let _ = tx
                .send(Ok(Bytes::from(format!("Upstream request error: {e}\n"))))
                .await;
            return;
        }
    };

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        let _ = tx
            .send(Ok(Bytes::from(format!(
                "Upstream returned {status}: {body}\n"
            ))))
            .await;
        return;
    }

    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        match item {
            Ok(chunk) => {
                if tx.send(Ok(chunk)).await.is_err() {
                    break;
                }
            }
            Err(e) => {
                let _ = tx
                    .send(Ok(Bytes::from(format!("Upstream stream error: {e}\n"))))
                    .await;
                break;
            }
        }
    }
}

async fn home() -> impl IntoResponse {
    Html("TIHC Serverless is running")
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    format!("Axum fallback for path {}", uri.path())
}

async fn stream_chat(headers: HeaderMap, Json(payload): Json<ChatRequest>) -> impl IntoResponse {
    let client = Client::new();

    if auth_required() {
        match get_bearer_token(&headers) {
            Some(token) => {
                let verify = verify_google_token(&client, &token)
                    .await
                    .and_then(|info| {
                        enforce_audience(&info)?;
                        enforce_workspace_domain(&info)?;
                        Ok(())
                    });

                if let Err(e) = verify {
                    warn!("[stream_chat] auth failed: {}", e);
                    return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
                }
            }
            None => {
                return (StatusCode::UNAUTHORIZED, "Missing bearer token").into_response();
            }
        }
    }

    let api_url = env_trimmed("TIDB_API_URL");
    let token = env_trimmed("TIDB_API_TOKEN");
    if api_url.is_empty() || token.is_empty() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Missing TIDB_API_URL or TIDB_API_TOKEN",
        )
            .into_response();
    }

    if payload.chat_engine.trim().to_lowercase() != "tidb" {
        info!(
            "[stream_chat] non-tidb engine '{}' received, forwarded as tidb proxy",
            payload.chat_engine
        );
    }

    let mut response = stream_response(move |tx| async move {
        proxy_stream(&client, api_url, token, &payload, tx).await;
    })
    .into_response();

    response
        .headers_mut()
        .insert(CONTENT_TYPE, "text/plain; charset=utf-8".parse().unwrap());
    response
        .headers_mut()
        .insert(CACHE_CONTROL, "no-store".parse().unwrap());

    response
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(
            env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string())
                .to_lowercase(),
        ),
    )
    .init();

    let app = Router::new()
        .route("/", get(home))
        .route("/api/stream_chat", post(stream_chat))
        .fallback(fallback)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));

    let app = ServiceBuilder::new().layer(VercelLayer).service(app);

    vercel_runtime::run(app).await
}
