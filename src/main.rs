use ml::{parse, request};

#[tokio::main]
async fn main() {
    let urls = ["https://jimaku.cc", "https://jimaku.cc/dramas"];

    for url in urls {
        let Ok(body) = request::get_body(url).await else {
            eprintln!("failed to get the request body");
            return;
        };

        let _entries = parse::parse(&body);
    }
}
