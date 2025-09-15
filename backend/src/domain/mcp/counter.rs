use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Counter {
    value: Arc<Mutex<i32>>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            value: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn increment(&self) -> i32 {
        let mut v = self.value.lock().await;
        *v += 1;
        *v
    }

    pub async fn decrement(&self) -> i32 {
        let mut v = self.value.lock().await;
        *v -= 1;
        *v
    }

    pub async fn get(&self) -> i32 {
        *self.value.lock().await
    }

    pub async fn analyze(&self, goal: i32, strategy: &str) -> (i32, i32, String) {
        let current = *self.value.lock().await;
        let diff = goal - current;
        (current, diff, strategy.to_string())
    }
}
