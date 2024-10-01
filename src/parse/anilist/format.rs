use std::fmt;

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

pub fn to_format(format: &str) -> Result<Format> {
    match format {
        "TV" => Ok(Format::TvShow),
        "Movie" => Ok(Format::Movie),
        "TV Short" => Ok(Format::TvShort),
        "Special" => Ok(Format::Special),
        "OVA" => Ok(Format::Ova),
        "ONA" => Ok(Format::Ona),
        "Music" => Ok(Format::Music),
        _ => bail!("Unknown format: {}", format),
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Format::TvShow => "TV",
            Format::Movie => "Movie",
            Format::TvShort => "TV Short",
            Format::Special => "Special",
            Format::Ova => "OVA",
            Format::Ona => "ONA",
            Format::Music => "Music",
        };
        write!(f, "{}", s)
    }
}
