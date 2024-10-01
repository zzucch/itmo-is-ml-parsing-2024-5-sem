use anyhow::{bail, Result};

#[derive(Debug)]
pub enum Genre {
    Action,
    Adventure,
    Comedy,
    Drama,
    Ecchi,
    Fantasy,
    Horror,
    MahouShoujo,
    Mecha,
    Music,
    Mystery,
    Psychological,
    Romance,
    SciFi,
    SliceOfLife,
    Sports,
    Supernatural,
    Thriller,
}

pub fn convert_genres(genres: &Vec<String>) -> Result<Vec<Genre>> {
    genres
        .into_iter()
        .map(|genre| match genre.as_str() {
            "Action" => Ok(Genre::Action),
            "Adventure" => Ok(Genre::Adventure),
            "Comedy" => Ok(Genre::Comedy),
            "Drama" => Ok(Genre::Drama),
            "Ecchi" => Ok(Genre::Ecchi),
            "Fantasy" => Ok(Genre::Fantasy),
            "Horror" => Ok(Genre::Horror),
            "Mahou Shoujo" => Ok(Genre::MahouShoujo),
            "Mecha" => Ok(Genre::Mecha),
            "Music" => Ok(Genre::Music),
            "Mystery" => Ok(Genre::Mystery),
            "Psychological" => Ok(Genre::Psychological),
            "Romance" => Ok(Genre::Romance),
            "Sci-Fi" => Ok(Genre::SciFi),
            "Slice of Life" => Ok(Genre::SliceOfLife),
            "Sports" => Ok(Genre::Sports),
            "Supernatural" => Ok(Genre::Supernatural),
            "Thriller" => Ok(Genre::Thriller),
            _ => bail!("Unknown genre: {}", genre),
        })
        .collect()
}
