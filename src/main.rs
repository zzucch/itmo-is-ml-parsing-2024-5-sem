use std::process::exit;

use anyhow::{anyhow, bail, Context, Result};
use headless_chrome::{Browser, LaunchOptionsBuilder};
use ml_parser::{
    convert::entry::get_tsv_entry,
    parse::{
        anilist::{self, entry::parse_anilist_entry},
        jimaku::{
            self,
            entry::parse_entries,
            file::{parse_files_data, FileData},
        },
    },
    request::{get_body, get_page_data_chrome},
    storage::save_to_tsv,
};

#[tokio::main]
async fn main() -> Result<()> {
    let urls = ["https://jimaku.cc", "https://jimaku.cc/dramas"];

    let entries = get_jimaku_entries(&urls)
        .await
        .context("Failed to get jimaku entries")?;

    let mut browser = get_new_browser()?;

    let mut current = 0;
    let mut failed_in_a_row = 0;

    for entry in &entries {
        let anilist_id = match entry.anilist_id {
            None => continue,
            Some(anilist_id) => anilist_id,
        };

        let files_data = get_jimaku_entry_files_data(&entry)
            .await
            .context("Failed to get jimaku entry files data")?;

        if files_data.is_empty() {
            continue;
        }

        let anilist_data = match get_anilist_entry(&mut browser, anilist_id).await {
            Ok(anilist_data) => anilist_data,
            Err(err) => {
                eprintln!("Failed to get anilist entry {anilist_id}: {:#}", err);
                failed_in_a_row += 1;

                if failed_in_a_row == 5 {
                    exit(1);
                }
                continue;
            }
        };

        failed_in_a_row = 0;

        let tsv_entry = get_tsv_entry(&entry, &files_data, &anilist_data)?;

        save_to_tsv(&tsv_entry, "./data/data.tsv")?;

        if current % 10 == 0 {
            println!("{current}");
        }
        current += 1;
    }

    Ok(())
}

fn get_new_browser() -> Result<Browser> {
    let launch_options = LaunchOptionsBuilder::default()
        .build()
        .map_err(|e| anyhow!("Failed to build launch options: {:?}", e))?;

    Browser::new(launch_options).map_err(|e| anyhow!("Failed to create browser: {:?}", e))
}

async fn get_anilist_entry(
    browser: &mut Browser,
    anilist_id: i32,
) -> Result<anilist::entry::Entry> {
    const URL: &str = "https://anilist.co/anime/";
    let url = URL.to_owned() + &anilist_id.to_string();

    let (head, body) = get_page_data_chrome_with_retry(browser, &url).await?;

    let anilist_entry =
        parse_anilist_entry(&head, &body).context(format!("Failed to parse request head"))?;

    Ok(anilist_entry)
}

async fn get_page_data_chrome_with_retry(
    browser: &mut Browser,
    url: &str,
) -> Result<(String, String)> {
    for _ in 0..10 {
        match get_page_data_chrome(&browser, url).await {
            Ok((head, body)) => return Ok((head, body)),
            Err(_) => {
                eprintln!("restarting browser");
                *browser = get_new_browser()?;
            }
        }
    }

    get_page_data_chrome(browser, url)
        .await
        .map_err(|e| anyhow!("Failed to get request head: {:?}", e))
}

async fn get_jimaku_entry_files_data(entry: &jimaku::entry::Entry) -> Result<Vec<FileData>> {
    const URL: &str = "https://jimaku.cc/entry/";
    let url = URL.to_owned() + &entry.id.to_string();

    let body = get_body(&url)
        .await
        .map_err(|e| anyhow!("Failed to get request body: {:?}", e))?;

    let files_data = parse_files_data(&body).context("Failed to parse request body")?;

    Ok(files_data)
}

async fn get_jimaku_entries(urls: &[&str]) -> Result<Vec<jimaku::entry::Entry>> {
    let tasks: Vec<_> = urls
        .iter()
        .map(|&url| {
            let url = url.to_string();

            tokio::spawn(async move {
                let body = get_body(&url)
                    .await
                    .map_err(|e| anyhow!("Failed to get request body: {:?}", e))?;

                let entries = parse_entries(&body).context("Failed to parse request body")?;

                let len = entries.len();
                println!("{len}");

                Ok::<Vec<jimaku::entry::Entry>, anyhow::Error>(entries)
            })
        })
        .collect();

    let mut all_entries = Vec::new();

    for task in tasks {
        match task.await {
            Ok(entries) => all_entries.extend(entries?),
            Err(err) => bail!("Task failed: {:?}", err),
        }
    }

    Ok(all_entries)
}
