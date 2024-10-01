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

pub fn convert_source(source: &str) -> Result<Source> {
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
