use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SteamDirError {
    #[error("Failed to get steam directory: {0}")]
    Fetch(String),
}

pub trait SteamDirectory {
    fn get_steam_dir(&self) -> Result<PathBuf, SteamDirError>;
}
