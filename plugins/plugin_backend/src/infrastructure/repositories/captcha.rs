use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait CaptchaRepository: Send + Sync {
    fn save(&self, session_id: &str, text: &str, expiry_seconds: u64);

    fn validate(&self, session_id: &str, input: &str) -> bool;

    fn remove(&self, session_id: &str);
}

#[derive(Debug, Clone, Default)]
struct StoredCaptchaInfo {
    text: String,
    created_at: u64,
}

pub struct InMemoryCaptchaRepository {
    captchas: RwLock<HashMap<String, StoredCaptchaInfo>>,
}

impl InMemoryCaptchaRepository {
    pub fn new() -> Self {
        Self {
            captchas: RwLock::new(HashMap::new()),
        }
    }

    fn cleanup_expired(&self, expiry_seconds: u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut captchas = self.captchas.write().unwrap();
        captchas.retain(|_, info| now - info.created_at <= expiry_seconds);
    }
}

impl CaptchaRepository for InMemoryCaptchaRepository {
    fn save(&self, session_id: &str, text: &str, expiry_seconds: u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        {
            let mut captchas = self.captchas.write().unwrap();
            captchas.insert(
                session_id.to_string(),
                StoredCaptchaInfo {
                    text: text.to_string(),
                    created_at: now,
                },
            );
        }

        // Clean expired captchas periodically
        self.cleanup_expired(expiry_seconds);
    }

    fn validate(&self, session_id: &str, input: &str) -> bool {
        let captchas = self.captchas.read().unwrap();

        if let Some(info) = captchas.get(session_id) {
            // Case insensitive comparison
            return info.text.to_lowercase() == input.to_lowercase();
        }

        false
    }

    fn remove(&self, session_id: &str) {
        let mut captchas = self.captchas.write().unwrap();
        captchas.remove(session_id);
    }
}
