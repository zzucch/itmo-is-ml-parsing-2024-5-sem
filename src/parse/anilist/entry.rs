use anyhow::{anyhow, bail, Context, Result};
use scraper::{Html, Selector};
use serde_json::Value;

use super::{
    format::{convert_format, Format},
    genre::{convert_genres, Genre},
    source::{convert_source, Source},
    status::{convert_status, Status},
};

#[derive(Debug)]
pub struct Entry {
    format: Option<Format>,
    status: Option<Status>,
    source: Option<Source>,
    genres: Vec<Genre>,
    episodes_amount: Option<i32>,
    time_required: String,
    start_date: Option<String>,
    end_date: Option<String>,
    rating_value: i32,
    rating_count: i32,
    production_companies: Option<Vec<i32>>,
    producers: Option<Vec<i32>>,
}

pub fn parse_anilist_entry(head_data: &str, body_data: &str) -> Result<Entry> {
    let mut entry = parse_head_data(head_data)?;

    let body_document = Html::parse_document(body_data);

    entry.status = match entry.status {
        Some(status) => Some(status),
        None => Some(parse_body_status(&body_document)?),
    };

    entry.format = match entry.format {
        Some(format) => Some(format),
        None => Some(parse_body_format(&body_document)?),
    };

    entry.source = match entry.source {
        Some(source) => Some(source),
        None => Some(parse_body_source(&body_document)?),
    };

    let airing_episodes_amount = parse_body_airing_episodes_amount(&body_document).ok();

    entry.episodes_amount = match airing_episodes_amount {
        Some(airing_amount) => Some(airing_amount),
        None => entry.episodes_amount,
    };

    println!("{entry:?}");

    Ok(entry)
}

fn parse_body_format(body_document: &Html) -> Result<Format> {
    let data_set_selector = Selector::parse("div.data-set")
        .map_err(|e| anyhow!("Failed to parse selector: {:?}", e))?;
    let type_selector =
        Selector::parse("div.type").map_err(|e| anyhow!("Failed to parse selector: {:?}", e))?;
    let value_selector =
        Selector::parse("div.value").map_err(|e| anyhow!("Failed to parse selector: {:?}", e))?;

    for data_set_element in body_document.select(&data_set_selector) {
        if let Some(type_element) = data_set_element.select(&type_selector).next() {
            if type_element
                .text()
                .collect::<Vec<_>>()
                .join("")
                .contains("Format")
            {
                if let Some(value_element) = data_set_element.select(&value_selector).next() {
                    let format_text = value_element
                        .text()
                        .collect::<Vec<_>>()
                        .join("")
                        .trim()
                        .to_string();
                    return convert_format(&format_text);
                }
            }
        }
    }
    Err(anyhow!("Format field not found"))
}

fn parse_body_source(body_document: &Html) -> Result<Source> {
    let data_set_selector = Selector::parse("div.data-set")
        .map_err(|e| anyhow!("Failed to parse selector: {:?}", e))?;
    let type_selector =
        Selector::parse("div.type").map_err(|e| anyhow!("Failed to parse selector: {:?}", e))?;
    let value_selector =
        Selector::parse("div.value").map_err(|e| anyhow!("Failed to parse selector: {:?}", e))?;

    for data_set_element in body_document.select(&data_set_selector) {
        if let Some(type_element) = data_set_element.select(&type_selector).next() {
            if type_element
                .text()
                .collect::<Vec<_>>()
                .join("")
                .contains("Source")
            {
                if let Some(value_element) = data_set_element.select(&value_selector).next() {
                    let source_text = value_element
                        .text()
                        .collect::<Vec<_>>()
                        .join("")
                        .trim()
                        .to_string();
                    return convert_source(&source_text);
                }
            }
        }
    }
    Err(anyhow!("Source field not found"))
}

fn parse_body_airing_episodes_amount(body_document: &Html) -> Result<i32> {
    let data_set_selector = Selector::parse("div.data-set")
        .map_err(|e| anyhow!("Failed to parse selector: {:?}", e))?;
    let type_selector =
        Selector::parse("div.type").map_err(|e| anyhow!("Failed to parse selector: {:?}", e))?;
    let countdown_selector = Selector::parse("div.countdown")
        .map_err(|e| anyhow!("Failed to parse selector: {:?}", e))?;

    for data_set_element in body_document.select(&data_set_selector) {
        if let Some(type_element) = data_set_element.select(&type_selector).next() {
            if type_element
                .text()
                .collect::<Vec<_>>()
                .join("")
                .contains("Airing")
            {
                if let Some(countdown_element) = data_set_element.select(&countdown_selector).next()
                {
                    let countdown_text = countdown_element
                        .text()
                        .collect::<Vec<_>>()
                        .join("")
                        .trim()
                        .to_string();
                    let episode_count_part = countdown_text
                        .split_whitespace()
                        .next()
                        .ok_or_else(|| anyhow!("Failed to extract episode count"))?;
                    let episode_count: i32 = episode_count_part
                        .replace("Ep", "")
                        .trim()
                        .parse()
                        .context("Failed to parse episode count to i32")?;
                    return Ok(episode_count);
                }
            }
        }
    }
    Err(anyhow!("Airing episodes amount field not found"))
}

fn parse_body_status(body_document: &Html) -> Result<Status> {
    let data_set_selector = Selector::parse("div.data-set")
        .map_err(|e| anyhow!("Failed to parse selector: {:?}", e))?;
    let type_selector =
        Selector::parse("div.type").map_err(|e| anyhow!("Failed to parse selector: {:?}", e))?;
    let value_selector =
        Selector::parse("div.value").map_err(|e| anyhow!("Failed to parse selector: {:?}", e))?;

    for data_set_element in body_document.select(&data_set_selector) {
        if let Some(type_element) = data_set_element.select(&type_selector).next() {
            if type_element
                .text()
                .collect::<Vec<_>>()
                .join("")
                .contains("Status")
            {
                if let Some(value_element) = data_set_element.select(&value_selector).next() {
                    let status_text = value_element
                        .text()
                        .collect::<Vec<_>>()
                        .join("")
                        .trim()
                        .to_string();
                    return convert_status(&status_text);
                }
            }
        }
    }
    Err(anyhow!("Status field not found"))
}

fn parse_head_data(head_data: &str) -> Result<Entry> {
    let document = Html::parse_document(head_data);
    let script_selector = Selector::parse("script[type=\"application/ld+json\"]")
        .map_err(|e| anyhow!("Failed to parse selector: {:?}", e))?;

    let script_content = document
        .select(&script_selector)
        .next()
        .context("Failed to find script tag")?
        .inner_html();

    let json_data: Value = serde_json::from_str(&script_content).context("Failed to parse JSON")?;

    let main_entity = &json_data["mainEntity"];

    let episodes_amount = main_entity["numberOfEpisodes"]
        .as_i64()
        .and_then(|num| i32::try_from(num).ok());

    let time_required = main_entity["timeRequired"]
        .as_str()
        .context(format!("Failed to get timeRequired from {:?}", main_entity))?
        .to_string();

    let start_date = main_entity["startDate"].as_str().map(|s| s.to_string());
    let end_date = main_entity["endDate"].as_str().map(|s| s.to_string());

    let rating_value = i32::try_from(
        main_entity["aggregateRating"]["ratingValue"]
            .as_i64()
            .context(format!("Failed to get ratingValue from {:?}", main_entity))?,
    )
    .context("Failed to convert ratingValue to i32")?;

    let rating_count = i32::try_from(
        main_entity["aggregateRating"]["ratingCount"]
            .as_i64()
            .context(format!("Failed to get ratingCount from {:?}", main_entity))?,
    )
    .context("Failed to convert ratingCount to i32")?;

    let production_companies = main_entity["productionCompany"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|company| {
                    company["@id"]
                        .as_str()
                        .context(format!(
                            "Failed to get @id for productionCompany from {:?}",
                            company
                        ))
                        .and_then(|id| extract_id_from_url(id))
                })
                .collect::<Result<Vec<_>>>()
        })
        .transpose()?;

    let producers = main_entity["producer"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|producer| {
                    producer["@id"]
                        .as_str()
                        .context(format!(
                            "Failed to get @id for producer from {:?}",
                            producer
                        ))
                        .and_then(|id| extract_id_from_url(id))
                })
                .collect::<Result<Vec<_>>>()
        })
        .transpose()?;

    let genres = main_entity["genre"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|genre| {
                    genre
                        .as_str()
                        .context(format!("Failed to get genre from {:?}", genre))
                        .map(|s| s.to_string())
                })
                .collect::<Result<Vec<_>>>()
        })
        .transpose()?
        .unwrap_or_default();

    let genres = convert_genres(&genres)?;

    let entry = Entry {
        format: None,
        status: None,
        source: None,
        genres,
        episodes_amount,
        time_required,
        start_date,
        end_date,
        rating_value,
        rating_count,
        production_companies,
        producers,
    };

    Ok(entry)
}

fn extract_id_from_url(url: &str) -> Result<i32> {
    let parts: Vec<&str> = url.split('/').collect();
    if let Some(id_part) = parts.iter().find(|&&part| part.parse::<i32>().is_ok()) {
        id_part
            .parse::<i32>()
            .context(format!("Failed to parse id from URL {url}"))
    } else {
        bail!("Failed to find numeric id in URL {url}")
    }
}
