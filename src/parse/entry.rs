use reqwest::Result;
use scraper::{Html, Selector};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Entry {
    #[serde(skip_deserializing)]
    pub id: i32,
    name: String,
    flags: i32,
    last_modified: i64,
    anilist_id: Option<i32>,
    tmdb_id: Option<String>,
    english_name: Option<String>,
    japanese_name: Option<String>,
}

pub fn parse_entries(data: &str) -> Result<Vec<Entry>> {
    let document = Html::parse_document(data);
    let selector = Selector::parse("div.entry").unwrap();

    let mut entries = Vec::new();

    for element in document.select(&selector) {
        let Some(mut entry) = parse_data_extra(&element) else {
            continue;
        };

        if let Some(id) = parse_id_from_element(&element) {
            entry.id = id;

            entries.push(entry);
        }
    }

    Ok(entries)
}

fn parse_data_extra(element: &scraper::ElementRef) -> Option<Entry> {
    let data_extra = element.value().attr("data-extra")?;

    serde_json::from_str(data_extra).unwrap()
}

fn parse_id_from_element(element: &scraper::ElementRef) -> Option<i32> {
    let link = element
        .select(&Selector::parse("a.table-data.file-name").unwrap())
        .next()?;

    let href = link.value().attr("href")?;

    href.split('/')
        .last()
        .and_then(|id_str| id_str.parse::<i32>().ok())
}
