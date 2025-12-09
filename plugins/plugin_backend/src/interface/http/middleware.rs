use axum::{
    extract::{Request, State},
    http::{Method, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::IntoResponse,
};
use std::sync::Arc;
use urlencoding::decode;

use crate::infrastructure::InfraState as AppState;

pub async fn auth_middleware(
    State(app_state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> impl IntoResponse {
    let auth_service = &app_state.auth_service;
    let token_service = &app_state.token_service;

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

    // Validate token
    match auth_service.validate_token(&token).await {
        Ok(claims) => {
            let user_id = claims.sub.parse::<i64>().unwrap_or(0);
            match token_service.is_token_active(&token, user_id).await {
                Ok(true) => {
                    request.extensions_mut().insert(claims);
                    request.extensions_mut().insert(token);
                    next.run(request).await
                }
                _ => (StatusCode::UNAUTHORIZED, "Token revoked or expired").into_response(),
            }
        }
        Err(_) => (StatusCode::UNAUTHORIZED, "Invalid or expired token").into_response(),
    }
}
