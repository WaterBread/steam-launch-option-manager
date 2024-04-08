use std::path::Path;

use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum FileError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn read_config_file(path: &Path) -> Result<String, FileError> {
    let file = std::fs::read_to_string(path);

    match file {
        Ok(file) => Ok(file.to_owned()),
        Err(e) => Err(FileError::Io(e)),
    }
}

pub fn save_config_file(path: &Path, content: String) -> Result<(), FileError> {
    let file = std::fs::write(path, content);

    match file {
        Ok(_) => Ok(()),
        Err(e) => Err(FileError::Io(e)),
    }
}
