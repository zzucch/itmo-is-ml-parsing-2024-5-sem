use headless_chrome::{Browser, LaunchOptionsBuilder};
use ml_parser::{
    //convert::entry::get_processed_entry,
    parse::jimaku::{
        entry::{parse_entries, Entry},
        file::{parse_files_data, FileData},
    },
    request,
};

#[tokio::main]
async fn main() {
    let urls = ["https://jimaku.cc", "https://jimaku.cc/dramas"];

    let entries = get_entries(&urls).await;

    let launch_options = LaunchOptionsBuilder::default().build().unwrap();
    let browser = Browser::new(launch_options).unwrap();

    let mut i = 0;

    for entry in entries {
        if entry.anilist_id.is_none() {
            continue;
        }

        let files_data = get_entry_files_data(&entry).await;

        if files_data.is_empty() {
            continue;
        }

        let _anilist_data = get_anilist_data(&browser, entry.anilist_id.unwrap()).await;

        // let _processed_entry = get_processed_entry(&entry, &files_data);

        println!("{i}");
        i += 1;
    }
}

async fn get_anilist_data(browser: &Browser, anilist_id: i32) {
    const URL: &str = "https://anilist.co/anime/";
    let url = URL.to_owned() + &anilist_id.to_string();

    let Ok(body) = request::get_body_chrome(browser, &url).await else {
        eprintln!("failed to get the request body for {url}");
        return;
    };

    println!("{body}")

    //   let Ok(files_data) = parse_anilist_data(&body) else {
    //       eprintln!("failed to parse request body for {url}:\n{body}");
    //       return Vec::new();
    //   };
    //
    //   files_data
}

async fn get_entry_files_data(entry: &Entry) -> Vec<FileData> {
    const URL: &str = "https://jimaku.cc/entry/";
    let url = URL.to_owned() + &entry.id.to_string();

    let Ok(body) = request::get_body(&url).await else {
        eprintln!("failed to get the request body for {url}");
        return Vec::new();
    };

    let Ok(files_data) = parse_files_data(&body) else {
        eprintln!("failed to parse request body for {url}:\n{body}");
        return Vec::new();
    };

    files_data
}

async fn get_entries(urls: &[&str]) -> Vec<Entry> {
    let tasks: Vec<_> = urls
        .iter()
        .map(|&url| {
            let url = url.to_string();

            tokio::spawn(async move {
                let Ok(body) = request::get_body(&url).await else {
                    eprintln!("failed to get the request body for {url}");
                    return Vec::new();
                };

                let Ok(entries) = parse_entries(&body) else {
                    eprintln!("failed to parse request body for {url}:\n{body}");
                    return Vec::new();
                };

                let len = entries.len();
                println!("{len}");

                entries
            })
        })
        .collect();

    let mut all_entries = Vec::new();

    for task in tasks {
        match task.await {
            Ok(entries) => all_entries.extend(entries),
            Err(err) => eprintln!("task failed: {err:?}"),
        }
    }

    all_entries
}
