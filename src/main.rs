use anyhow::{anyhow, bail, Context, Result};
use headless_chrome::{Browser, LaunchOptionsBuilder};
use ml_parser::{
    parse::{
        anilist::{self, entry::parse_anilist_entry},
        jimaku::{
            self,
            entry::parse_entries,
            file::{parse_files_data, FileData},
        },
    },
    request::{get_body, get_head_chrome},
};

#[tokio::main]
async fn main() -> Result<()> {
    let urls = ["https://jimaku.cc", "https://jimaku.cc/dramas"];

    let entries = get_jimaku_entries(&urls)
        .await
        .context("Failed to get jimaku entries")?;

    let launch_options = LaunchOptionsBuilder::default()
        .build()
        .map_err(|e| anyhow!("Failed to build launch options: {:?}", e))?;
    let browser =
        Browser::new(launch_options).map_err(|e| anyhow!("Failed to create browser: {:?}", e))?;

    let mut i = 0;

    for entry in entries {
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

        let _anilist_data = get_anilist_entry(&browser, anilist_id)
            .await
            .context(format!("Failed to get anilist entry {}", anilist_id))?;

        // let _processed_entry = get_processed_entry(&entry, &files_data);

        println!("{i}");
        i += 1;
    }

    Ok(())
}

async fn get_anilist_entry(browser: &Browser, anilist_id: i32) -> Result<anilist::entry::Entry> {
    const URL: &str = "https://anilist.co/anime/";
    let url = URL.to_owned() + &anilist_id.to_string();

    let head = get_head_chrome(browser, &url)
        .await
        .map_err(|e| anyhow!("Failed to get request head: {:?}", e))?;

    let anilist_entry =
        parse_anilist_entry(&head).context(format!("Failed to parse request head: {}", head))?;

    println!("{anilist_entry:?}");

    Ok(anilist_entry)
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
