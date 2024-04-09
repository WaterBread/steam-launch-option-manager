use std::path::PathBuf;

use steamlocate::SteamDir;

use crate::traits::steam_dir::{SteamDirError, SteamDirectory};

pub struct SteamLocateSteamDir;

impl SteamDirectory for SteamLocateSteamDir {
    fn get_steam_dir(&self) -> Result<PathBuf, SteamDirError> {
        match SteamDir::locate() {
            Some(steamdir) => Ok(steamdir.path),
            None => panic!("Couldn't locate Steam on this computer!"),
        }
    }
}
