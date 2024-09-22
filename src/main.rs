use ml::{parse, request};
use tokio;

#[tokio::main]
async fn main() {
    let url = "https://jimaku.cc";

    let Ok(body) = request::get_body(url).await else {
        todo!()
    };

    parse::parse(&body);
}
