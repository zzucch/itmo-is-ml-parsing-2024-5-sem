use ml_parser::{parse, request};

#[tokio::main]
async fn main() {
    let urls = ["https://jimaku.cc", "https://jimaku.cc/dramas"];

    let tasks: Vec<_> = urls
        .iter()
        .map(|&url| {
            let url = url.to_string();
            tokio::spawn(async move {
                let Ok(body) = request::get_body(&url).await else {
                    eprintln!("failed to get the request body for {}", url);
                    return;
                };

                let entries = parse::parse(&body);
            })
        })
        .collect();

    for task in tasks {
        if let Err(e) = task.await {
            eprintln!("task failed: {:?}", e);
        }
    }
}
