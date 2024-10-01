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
    let format_lower = format.to_lowercase();
    if format_lower.contains("movie") {
        return Ok(Format::Movie); // 10259 is 'Movie (Chinese)'
    }

    if format_lower.contains("ona") {
        return Ok(Format::Ona); // 103350 is 'ONA (Chinese)'
    }

    match format {
        "TV" => Ok(Format::TvShort),
        "Movie" => Ok(Format::Movie),
        "TV Short" => Ok(Format::TvShort),
        "Special" => Ok(Format::Special),
        "OVA" => Ok(Format::Ova),
        "Music" => Ok(Format::Music),
        _ => bail!("Unknown format: {}", format),
    }
}
