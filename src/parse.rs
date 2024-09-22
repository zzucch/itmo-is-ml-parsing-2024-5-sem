use scraper::{Html, Selector};

#[derive(Debug, serde::Deserialize)]
pub struct Entry {
    name: String,
    flags: u64,
    last_modified: u64,
    anilist_id: Option<u64>,
    tmdb_id: Option<String>,
    english_name: Option<String>,
    japanese_name: Option<String>,
}

pub fn parse(data: &str) {
    let document = Html::parse_document(data);
    let selector = Selector::parse("div.entry").unwrap();

    for element in document.select(&selector) {
        if let Some(data_extra) = element.value().attr("data-extra") {
            let entry = parse_data_extra(data_extra);
            println!("{:?}", entry);
        }
    }
}

fn parse_data_extra(data_extra: &str) -> Entry {
    let data_extra_decoded = data_extra.replace("&quot;", "\"");
    serde_json::from_str(&data_extra_decoded).unwrap()
}
