use anyhow::{bail, Result};

#[derive(Debug)]
pub enum Format {
    TvShow,
    Movie,
    TvShort,
    Special,
    Ova,
    Ona,
    Music,
}

pub fn convert_format(format: &str) -> Result<Format> {
    match format {
        "TV" => Ok(Format::TvShort),
        "Movie" => Ok(Format::Movie),
        "TV Short" => Ok(Format::TvShort),
        "Special" => Ok(Format::Special),
        "OVA" => Ok(Format::Ova),
        "ONA" => Ok(Format::Ona),
        "Music" => Ok(Format::Music),
        _ => bail!("Unknown format: {}", format),
    }
}
