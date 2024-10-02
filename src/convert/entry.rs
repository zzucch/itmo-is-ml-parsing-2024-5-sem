use crate::parse::{
    anilist::{self, genre::Genre},
    jimaku::{self, file::FileData},
};
use anyhow::{bail, Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;

use super::flags::EntryFlags;

#[derive(Debug, Serialize)]
pub struct TsvEntry {
    pub name_romaji: String,
    pub name_english: String,
    pub name_japanese: String,
    pub is_unverified: bool,
    pub is_external: bool,
    pub is_movie: bool,
    pub is_adult: bool,

    pub is_action: bool,
    pub is_adventure: bool,
    pub is_comedy: bool,
    pub is_drama: bool,
    pub is_ecchi: bool,
    pub is_fantasy: bool,
    pub is_horror: bool,
    pub is_mahou_shoujo: bool,
    pub is_mecha: bool,
    pub is_music: bool,
    pub is_mystery: bool,
    pub is_psychological: bool,
    pub is_romance: bool,
    pub is_sci_fi: bool,
    pub is_slice_of_life: bool,
    pub is_sports: bool,
    pub is_supernatural: bool,
    pub is_thriller: bool,

    pub format: String,
    pub status: String,
    pub source: String,
    pub episodes_amount: i32,
    pub time_required: i64,
    pub start_date: i64,
    pub end_date: i64,
    pub rating_value: i32,
    pub rating_count: i32,
    pub company_production: i32,
    pub company_producer: i32,
    pub company_creator: i32,

    pub last_modified: i64,
    pub file_modified_first: i64,
    pub file_modified_last: i64,
    pub file_modified_median: i64,
    pub filesize_min: i64,
    pub filesize_max: i64,
    pub filesize_median: i64,
}

pub fn get_tsv_entry(
    jimaku_entry: &jimaku::entry::Entry,
    jimaku_files_info: &[jimaku::file::FileData],
    anilist_entry: &anilist::entry::Entry,
) -> Result<TsvEntry> {
    let file_stats = calculate_file_stats(jimaku_files_info);

    Ok(TsvEntry {
        name_romaji: jimaku_entry.name.clone(),
        name_english: match &jimaku_entry.english_name {
            Some(name) => name.to_string(),
            None => "?".to_string(),
        },
        name_japanese: match &jimaku_entry.japanese_name {
            Some(name) => name.to_string(),
            None => "?".to_string(),
        },
        is_unverified: EntryFlags::new(jimaku_entry.flags).is_unverified(),
        is_external: EntryFlags::new(jimaku_entry.flags).is_external(),
        is_movie: EntryFlags::new(jimaku_entry.flags).is_movie(),
        is_adult: EntryFlags::new(jimaku_entry.flags).is_adult(),
        format: anilist_entry.format.to_string(),
        status: anilist_entry.status.to_string(),
        source: match &anilist_entry.source {
            Some(source) => source.to_string(),
            None => "?".to_string(),
        },
        episodes_amount: match anilist_entry.episodes_amount {
            Some(episodes_amount) => episodes_amount,
            None => 0,
        },
        time_required: match &anilist_entry.time_required {
            Some(time) => parse_time(time)?,
            None => 0,
        },
        start_date: match &anilist_entry.start_date {
            Some(start_date) => parse_date(start_date)?,
            None => 0,
        },
        end_date: match &anilist_entry.end_date {
            Some(end_date) => parse_date(end_date)?,
            None => 0,
        },
        rating_value: anilist_entry.rating_value,
        rating_count: anilist_entry.rating_count,
        company_production: match anilist_entry.production_company {
            Some(company) => company,
            None => 0,
        },
        company_producer: match anilist_entry.producer {
            Some(company) => company,
            None => 0,
        },
        company_creator: match anilist_entry.creator {
            Some(company) => company,
            None => 0,
        },
        last_modified: jimaku_entry.last_modified,
        file_modified_first: file_stats.file_modified_first,
        file_modified_last: file_stats.file_modified_last,
        file_modified_median: file_stats.file_modified_median,
        filesize_min: file_stats.filesize_min,
        filesize_max: file_stats.filesize_max,
        filesize_median: file_stats.filesize_median,
        is_action: anilist_entry.genres.contains(&Genre::Action),
        is_adventure: anilist_entry.genres.contains(&Genre::Adventure),
        is_comedy: anilist_entry.genres.contains(&Genre::Comedy),
        is_drama: anilist_entry.genres.contains(&Genre::Drama),
        is_ecchi: anilist_entry.genres.contains(&Genre::Ecchi),
        is_fantasy: anilist_entry.genres.contains(&Genre::Fantasy),
        is_horror: anilist_entry.genres.contains(&Genre::Horror),
        is_mahou_shoujo: anilist_entry.genres.contains(&Genre::MahouShoujo),
        is_mecha: anilist_entry.genres.contains(&Genre::Mecha),
        is_music: anilist_entry.genres.contains(&Genre::Music),
        is_mystery: anilist_entry.genres.contains(&Genre::Mystery),
        is_psychological: anilist_entry.genres.contains(&Genre::Psychological),
        is_romance: anilist_entry.genres.contains(&Genre::Romance),
        is_sci_fi: anilist_entry.genres.contains(&Genre::SciFi),
        is_slice_of_life: anilist_entry.genres.contains(&Genre::SliceOfLife),
        is_sports: anilist_entry.genres.contains(&Genre::Sports),
        is_supernatural: anilist_entry.genres.contains(&Genre::Supernatural),
        is_thriller: anilist_entry.genres.contains(&Genre::Thriller),
    })
}

#[derive(Debug)]
struct FileStats {
    file_modified_first: i64,
    file_modified_last: i64,
    file_modified_median: i64,
    filesize_min: i64,
    filesize_max: i64,
    filesize_median: i64,
}

fn calculate_file_stats(files_info: &[FileData]) -> FileStats {
    let mut modified_times: Vec<i64> = files_info
        .iter()
        .map(|file| {
            DateTime::parse_from_rfc3339(&file.last_modified)
                .map(|dt| dt.timestamp())
                .unwrap_or(0)
        })
        .collect();
    let mut file_sizes: Vec<i64> = files_info.iter().map(|file| file.size).collect();

    modified_times.sort_unstable();
    file_sizes.sort_unstable();

    let file_modified_first = *modified_times.first().unwrap_or(&0);
    let file_modified_last = *modified_times.last().unwrap_or(&0);
    let file_modified_median = if modified_times.len() % 2 == 0 {
        (modified_times[modified_times.len() / 2 - 1] + modified_times[modified_times.len() / 2])
            / 2
    } else {
        modified_times[modified_times.len() / 2]
    };

    let filesize_min = *file_sizes.first().unwrap_or(&0);
    let filesize_max = *file_sizes.last().unwrap_or(&0);
    let filesize_median = if file_sizes.len() % 2 == 0 {
        (file_sizes[file_sizes.len() / 2 - 1] + file_sizes[file_sizes.len() / 2]) / 2
    } else {
        file_sizes[file_sizes.len() / 2]
    };

    FileStats {
        file_modified_first,
        file_modified_last,
        file_modified_median,
        filesize_min,
        filesize_max,
        filesize_median,
    }
}

fn parse_time(time: &str) -> Result<i64> {
    let duration: iso8601_duration::Duration = match time.parse() {
        Ok(duration) => duration,
        Err(err) => bail!(format!("Failed to parse ISO8601 duration: {err:?}")),
    };

    let duration = duration
        .to_chrono()
        .context("Failed to convert duration to chrono")?;
    Ok(duration.num_milliseconds())
}

fn parse_date(date: &str) -> Result<i64> {
    let naive_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .context(format!("Failed to parse naive date from string {date}"))?;
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(
        naive_date
            .and_hms_opt(0, 0, 0)
            .context("Failed to make NaiveDateTime")?,
        Utc,
    );
    Ok(datetime.timestamp())
}
