use anyhow::{anyhow, Context, Result};
use scraper::{Html, Selector};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Entry {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub flags: u32,
    pub last_modified: i64,
    pub anilist_id: Option<i32>,
    _tmdb_id: Option<String>,
    pub english_name: Option<String>,
    pub japanese_name: Option<String>,
}

pub fn parse_entries(data: &str) -> Result<Vec<Entry>> {
    let document = Html::parse_document(data);
    let selector =
        Selector::parse("div.entry").map_err(|e| anyhow!("Failed to parse selector: {:?}", e))?;

    let mut entries = Vec::new();

    for element in document.select(&selector) {
        let mut entry = parse_data_extra(&element).context("Failed to parse data-extra")?;

        if let Some(id) =
            parse_id_from_element(&element).context("Failed to parse ID from element")?
        {
            entry.id = id;
            entries.push(entry);
        }
    }

    Ok(entries)
}

fn parse_data_extra(element: &scraper::ElementRef) -> Result<Entry> {
    let data_extra = element
        .value()
        .attr("data-extra")
        .context("Failed to get data-extra attribute")?;
    Ok(serde_json::from_str(data_extra).context("Failed to parse JSON")?)
}

fn parse_id_from_element(element: &scraper::ElementRef) -> Result<Option<i32>> {
    let link = element
        .select(
            &Selector::parse("a.table-data.file-name")
                .map_err(|e| anyhow!("Failed to parse selector for link: {:?}", e))?,
        )
        .next()
        .context("Failed to find link element")?;

    let href = link
        .value()
        .attr("href")
        .context("Failed to get href attribute")?;

    Ok(href
        .split('/')
        .last()
        .and_then(|id_str| id_str.parse::<i32>().ok()))
}
