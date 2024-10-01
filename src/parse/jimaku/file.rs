use anyhow::{anyhow, Context};
use scraper::{Html, Selector};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileData {
    pub name: String,
    pub size: i64,
    pub last_modified: String,
}

pub fn parse_files_data(body: &str) -> anyhow::Result<Vec<FileData>> {
    let document = Html::parse_document(body);
    let selector =
        Selector::parse("div.entry").map_err(|e| anyhow!("Failed to parse selector: {:?}", e))?;

    let mut files_data = Vec::new();

    for element in document.select(&selector) {
        if let Some(file_data) = parse_file_data(&element).context("Failed to parse file data")? {
            files_data.push(file_data);
        }
    }

    Ok(files_data)
}

fn parse_file_data(element: &scraper::ElementRef) -> anyhow::Result<Option<FileData>> {
    let data_extra = element
        .value()
        .attr("data-extra")
        .context("Failed to get data-extra attribute")?;

    Ok(serde_json::from_str(data_extra).context("Failed to parse JSON")?)
}
