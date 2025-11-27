use axum::{
    extract::{Request, State},
    http::{Method, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::IntoResponse,
};
use std::sync::Arc;
use tracing::error;
use urlencoding::decode;

use crate::infrastructure::InfraState as AppState;

pub async fn auth_middleware(
    State(app_state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> impl IntoResponse {
    let auth_service = &app_state.auth_service;

    if request.method() == Method::OPTIONS {
        return next.run(request).await;
    }
    // Extract token from Authorization header
    let mut token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_header| {
            if auth_header.starts_with("Bearer ") {
                Some(auth_header[7..].to_string())
            } else {
                None
            }
        });

    if token.is_none() {
        if let Some(query) = request.uri().query() {
            for pair in query.split('&') {
                let mut parts = pair.splitn(2, '=');
                if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                    if key == "token" {
                        if let Ok(decoded) = decode(value) {
                            let candidate = decoded.into_owned();
                            if !candidate.is_empty() {
                                token = Some(candidate);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    let token = match token {
        Some(token) => token,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                "Missing or invalid authorization token",
            )
                .into_response();
        }
    };

    match app_state.auth_token_store.is_token_active(&token).await {
        Ok(true) => {}
        Ok(false) => {
            return (
                StatusCode::UNAUTHORIZED,
                "Token has expired or been revoked",
            )
                .into_response();
        }
        Err(err) => {
            error!("token persistence lookup failed: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Token verification failed",
            )
                .into_response();
        }
    }

    // Validate token
    match auth_service.validate_token(&token).await {
        Ok(claims) => {
            // Add claims to request extensions
            request.extensions_mut().insert(claims);
            // Also insert raw access token so downstream handlers (eg. logout) can access it
            request.extensions_mut().insert(token);
            next.run(request).await
        }
        Err(_) => (StatusCode::UNAUTHORIZED, "Invalid or expired token").into_response(),
    }
}
