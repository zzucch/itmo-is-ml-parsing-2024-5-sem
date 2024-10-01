use anyhow::{Context, Result};
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::convert::entry::TsvEntry;

static HEADER_WRITTEN: AtomicBool = AtomicBool::new(false);

pub fn save_to_tsv<P: AsRef<Path>>(entry: &TsvEntry, file_path: P) -> Result<()> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .context("Failed to open file")?;

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(!HEADER_WRITTEN.load(Ordering::SeqCst))
        .from_writer(file);

    wtr.serialize(entry).context("Failed to serialize")?;
    wtr.flush().context("Failed to flush")?;

    HEADER_WRITTEN.store(true, Ordering::SeqCst);

    Ok(())
}
