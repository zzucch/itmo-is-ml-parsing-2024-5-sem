use ml::{parse, request};
use tokio;

#[tokio::main]
async fn main() {
    let url = "https://jimaku.cc";

    match request::get_body(url).await {
        Ok(body) => parse::parse(&body),
        Err(e) => eprintln!("Error: {}", e),
    }
}
