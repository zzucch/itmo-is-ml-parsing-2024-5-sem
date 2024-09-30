use reqwest::Result;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct FileInfo {
    pub name: String,
    pub size: i64,
    pub last_modified: String,
}

pub fn parse_files_info(body: &str) -> Result<Vec<FileInfo>> {
    let document = Html::parse_document(body);
    let selector = Selector::parse("div.entry").unwrap();

    let mut files_info = Vec::new();

    for element in document.select(&selector) {
        if let (Some(name), Some(size), Some(last_modified)) = (
            parse_file_name(&element),
            parse_file_size(&element),
            parse_file_modified(&element),
        ) {
            files_info.push(FileInfo {
                name,
                size,
                last_modified,
            });
        }
    }

    Ok(files_info)
}

fn parse_file_name(element: &scraper::ElementRef) -> Option<String> {
    element
        .select(&Selector::parse("a.table-data.file-name").unwrap())
        .next()
        .map(|e| e.inner_html())
}

fn parse_file_size(element: &scraper::ElementRef) -> Option<i64> {
    element
        .select(&Selector::parse("span.table-data.file-size").unwrap())
        .next()
        .and_then(|e| {
            let size_str = e.inner_html();
            if size_str.ends_with(" kB") {
                size_str[..size_str.len() - 3]
                    .trim()
                    .parse::<f64>()
                    .ok()
                    .map(|size| (size * 1024.0) as i64)
            } else {
                None
            }
        })
}

fn parse_file_modified(element: &scraper::ElementRef) -> Option<String> {
    element
        .select(&Selector::parse("span.table-data.file-modified").unwrap())
        .next()
        .map(|e| e.inner_html())
}
