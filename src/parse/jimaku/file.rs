use reqwest::Result;
use scraper::{Html, Selector};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileData {
    pub name: String,
    pub size: i64,
    pub last_modified: String,
}

pub fn parse_files_data(body: &str) -> Result<Vec<FileData>> {
    let document = Html::parse_document(body);
    let selector = Selector::parse("div.entry").unwrap();

    let mut files_data = Vec::new();

    for element in document.select(&selector) {
        if let Some(file_data) = parse_file_data(&element) {
            files_data.push(file_data);
        }
    }

    Ok(files_data)
}

fn parse_file_data(element: &scraper::ElementRef) -> Option<FileData> {
    let data_extra = element.value().attr("data-extra")?;

    serde_json::from_str(data_extra).ok()
}
