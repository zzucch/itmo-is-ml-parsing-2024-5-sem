use std::fmt;

use anyhow::{bail, Result};

#[derive(Debug)]
pub enum Source {
    Original,
    Manga,
    LightNovel,
    WebNovel,
    Novel,
    Anime,
    VisualNovel,
    VideoGame,
    Doujinshi,
    Comic,
    LiveAction,
    Game,
    MultimediaProject,
    Other,
}

pub fn to_source(source: &str) -> Result<Source> {
    match source {
        "Original" => Ok(Source::Original),
        "Manga" => Ok(Source::Manga),
        "Light Novel" => Ok(Source::LightNovel),
        "Web Novel" => Ok(Source::WebNovel),
        "Novel" => Ok(Source::Novel),
        "Anime" => Ok(Source::Anime),
        "Visual Novel" => Ok(Source::VisualNovel),
        "Video Game" => Ok(Source::VideoGame),
        "Doujinshi" => Ok(Source::Doujinshi),
        "Comic" => Ok(Source::Comic),
        "Live Action" => Ok(Source::LiveAction),
        "Game" => Ok(Source::Game),
        "Multimedia Project" => Ok(Source::MultimediaProject),
        "Other" => Ok(Source::Other),
        _ => bail!("Unknown source: {}", source),
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Source::Original => "Original",
            Source::Manga => "Manga",
            Source::LightNovel => "Light Novel",
            Source::WebNovel => "Web Novel",
            Source::Novel => "Novel",
            Source::Anime => "Anime",
            Source::VisualNovel => "Visual Novel",
            Source::VideoGame => "Video Game",
            Source::Doujinshi => "Doujinshi",
            Source::Comic => "Comic",
            Source::LiveAction => "Live Action",
            Source::Game => "Game",
            Source::MultimediaProject => "Multimedia Project",
            Source::Other => "Other",
        };
        write!(f, "{}", s)
    }
}
