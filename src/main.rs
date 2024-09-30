use std::sync::Arc;
use tokio::time::Duration;

use ml_parser::{
    parse,
    request::{self, RateLimiter},
};

#[tokio::main]
async fn main() {
    let urls = ["https://jimaku.cc", "https://jimaku.cc/dramas"];

    let rate_limiter = Arc::new(RateLimiter::new(Duration::from_secs(1)));

    let tasks: Vec<_> = urls
        .iter()
        .map(|&url| {
            let url = url.to_string();
            let rate_limiter = Arc::clone(&rate_limiter);

            tokio::spawn(async move {
                let Ok(body) = request::get_body(&url, rate_limiter).await else {
                    eprintln!("failed to get the request body for {url}");
                    return;
                };

                let _entries = parse::parse(&body);
            })
        })
        .collect();

    for task in tasks {
        if let Err(err) = task.await {
            eprintln!("task failed: {err:?}");
        }
    }
}
