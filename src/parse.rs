use scraper::{Html, Selector};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Entry {
    #[serde(skip_deserializing)]
    id: i32,
    name: String,
    flags: i32,
    last_modified: i64,
    anilist_id: Option<i32>,
    tmdb_id: Option<String>,
    english_name: Option<String>,
    japanese_name: Option<String>,
}

pub fn parse(data: &str) {
    let document = Html::parse_document(data);
    let selector = Selector::parse("div.entry").unwrap();

    for element in document.select(&selector) {
        let data_extra = match element.value().attr("data-extra") {
            Some(data) => data,
            None => continue,
        };

        let mut entry = parse_data_extra(data_extra);

        if let Some(id) = parse_id_from_element(&element) {
            entry.id = id;
        } else {
            continue;
        }

        println!("{:?}", entry);
    }
}

fn parse_data_extra(data_extra: &str) -> Entry {
    let data_extra_decoded = data_extra.replace("&quot;", "\"");
    serde_json::from_str(&data_extra_decoded).unwrap()
}

fn parse_id_from_element(element: &scraper::ElementRef) -> Option<i32> {
    let link = match element
        .select(&Selector::parse("a.table-data.file-name").unwrap())
        .next()
    {
        Some(link) => link,
        None => return None,
    };

    let href = match link.value().attr("href") {
        Some(href) => href,
        None => return None,
    };

    href.split('/').last().and_then(|s| s.parse::<i32>().ok())
}
