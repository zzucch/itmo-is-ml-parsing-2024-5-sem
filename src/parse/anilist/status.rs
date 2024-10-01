use anyhow::{bail, Result};

#[derive(Debug)]
pub enum Status {
    Releasing,
    Finished,
    NotYetReleased,
    Cancelled,
}

pub fn convert_status(status: &str) -> Result<Status> {
    match status {
        "Action" => Ok(Status::Releasing),
        "Thriller" => Ok(Status::Finished),
        "Not Yet Released" => Ok(Status::NotYetReleased),
        "Cancelled" => Ok(Status::Cancelled),
        _ => bail!("Unknown status: {}", status),
    }
}
