use axum::{Json, response::IntoResponse, http::StatusCode, routing::{get}, Router};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use once_cell::sync::Lazy;


const SUPPORTED_LANGS: [&str; 2] = ["zh", "en"];
static LANG: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("en".to_string()));



#[derive(Serialize)]
struct LangResp {
    lang: String,
}

#[derive(Deserialize)]
pub struct LangSet {
    lang: String,
}

pub async fn get_lang() -> impl IntoResponse {
    let lang = LANG.lock().unwrap().clone();
    Json(LangResp { lang })
}

pub async fn set_lang(Json(payload): Json<LangSet>) -> impl IntoResponse {
    if SUPPORTED_LANGS.contains(&payload.lang.as_str()) {
        *LANG.lock().unwrap() = payload.lang;
    }
    StatusCode::OK
}

pub fn lang_router() -> Router {
    Router::new()
        .route("/lang", get(get_lang).post(set_lang))
}
