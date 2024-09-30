use std::sync::Arc;

use tokio::{
    sync::Mutex,
    time::{sleep, Duration, Instant},
};

pub struct RateLimiter {
    last_request_time: Mutex<Instant>,
    offset: Duration,
}

impl RateLimiter {
    #[must_use]
    pub fn new(duration: Duration) -> Self {
        Self {
            last_request_time: Mutex::new(Instant::now()),
            offset: duration,
        }
    }

    pub async fn enforce_rate_limit(&self) {
        let mut last_request_time = self.last_request_time.lock().await;

        let now = Instant::now();
        let elapsed = now.duration_since(*last_request_time);

        if elapsed < self.offset {
            sleep(self.offset - elapsed).await;
        }
        *last_request_time = Instant::now();
    }
}

pub async fn get_body(url: &str, rate_limiter: Arc<RateLimiter>) -> Result<String, reqwest::Error> {
    rate_limiter.enforce_rate_limit().await;

    let response = reqwest::get(url).await?;

    let body = response.text().await?;

    Ok(body)
}
