use std::fmt;

use anyhow::{bail, Result};

#[derive(Debug)]
pub enum Status {
    Releasing,
    Finished,
    NotYetReleased,
    Cancelled,
}

pub fn to_status(status: &str) -> Result<Status> {
    match status {
        "Releasing" => Ok(Status::Releasing),
        "Finished" => Ok(Status::Finished),
        "Not Yet Released" => Ok(Status::NotYetReleased),
        "Cancelled" => Ok(Status::Cancelled),
        _ => bail!("Unknown status: {}", status),
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Status::Releasing => "Releasing",
            Status::Finished => "Finished",
            Status::NotYetReleased => "Not Yet Released",
            Status::Cancelled => "Cancelled",
        };
        write!(f, "{}", s)
    }
}
