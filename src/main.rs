use std::sync::Arc;
use tokio::time::Duration;

use ml_parser::{
    parse::{self, Entry},
    request::{self, RateLimiter},
};

#[tokio::main]
async fn main() {
    let urls = ["https://jimaku.cc", "https://jimaku.cc/dramas"];

    let rate_limiter = Arc::new(RateLimiter::new(Duration::from_secs(0)));
    let entries = get_entries(&urls, rate_limiter).await;

    println!("entries: {entries:?}");
}

async fn get_entries(urls: &[&str], rate_limiter: Arc<RateLimiter>) -> Vec<Entry> {
    let tasks: Vec<_> = urls
        .iter()
        .map(|&url| {
            let url = url.to_string();
            let rate_limiter = Arc::clone(&rate_limiter);

            tokio::spawn(async move {
                let Ok(body) = request::get_body(&url, rate_limiter).await else {
                    eprintln!("failed to get the request body for {url}");
                    return Vec::new();
                };

                let Ok(entries) = parse::parse(&body) else {
                    eprintln!("failed to parse request body for {url}:\n{body}");
                    return Vec::new();
                };

                entries
            })
        })
        .collect();

    let mut all_entries = Vec::new();

    for task in tasks {
        match task.await {
            Ok(entries) => all_entries.extend(entries),
            Err(err) => eprintln!("task failed: {err:?}"),
        }
    }

    all_entries
}
