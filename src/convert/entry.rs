use crate::parse::jimaku::{entry::Entry, file::FileInfo};

#[derive(Debug)]
pub struct ProcessedEntry {
    id: i32,
    name_romaji: String,
    name_english: Option<String>,
    name_japanese: Option<String>,
    episode_amount: i32,
    episode_duration: i32,
    airing_start_date: i32,
    airing_end_date: i32,
    airing_season: String,
    weighted_score: i32,
    mean_score: i32,
    // status: Status,
    popularity: i32,
    favourites: i32,
    // studio: Studio,
    // source: Source,
    is_movie: bool,
    is_unverified: bool,
    is_external: bool,
    is_adult: bool,
    last_modified: i64,
    file_modified_first: i64,
    file_modified_last: i64,
    file_modified_median: i64,
    filesize_min: i64,
    filesize_max: i64,
    filesize_median: i64,
}

pub fn get_processed_entry(entry: &Entry, files_info: &[FileInfo]) -> ProcessedEntry {
    println!("{entry} {:files_info?}");
}
