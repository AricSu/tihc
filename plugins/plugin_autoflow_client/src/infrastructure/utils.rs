use std::time::Duration;

use reqwest::{Client, RequestBuilder};

use crate::domain::{AutoflowConfig, AutoflowError};

pub fn trim_trailing_slash(input: &str) -> String {
    let trimmed = input.trim_end_matches('/');
    if trimmed.is_empty() {
        input.to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn stream_timeout_seconds(config: &AutoflowConfig) -> u64 {
    if config.timeout_seconds == 0 {
        120
    } else {
        config.timeout_seconds.saturating_mul(4).max(120)
    }
}

pub fn truncate_debug(input: &str) -> String {
    const MAX_LEN: usize = 200;
    if input.len() <= MAX_LEN {
        input.to_string()
    } else {
        let mut truncated = String::new();
        for (idx, ch) in input.char_indices() {
            if idx >= MAX_LEN {
                break;
            }
            truncated.push(ch);
        }
        truncated.push_str("...<truncated>");
        truncated
    }
}

pub fn apply_auth(builder: RequestBuilder, api_key: Option<&str>) -> RequestBuilder {
    if let Some(key) = api_key {
        builder.bearer_auth(key)
    } else {
        builder
    }
}

pub fn build_http_client(config: &AutoflowConfig) -> Result<Client, reqwest::Error> {
    let timeout = if config.timeout_seconds == 0 {
        30
    } else {
        config.timeout_seconds
    };

    Client::builder()
        .timeout(Duration::from_secs(timeout))
        .build()
}

pub fn map_reqwest_error(err: reqwest::Error) -> AutoflowError {
    if err.is_timeout() {
        AutoflowError::Timeout
    } else {
        AutoflowError::HttpError(err)
    }
}
