use std::path::{Path, PathBuf};

use thiserror;

use crate::traits::config_reader::{ConfigError, ConfigReader};

#[derive(thiserror::Error, Debug)]
pub enum FileError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct ConfigFileRepo {
    pub file_path: PathBuf,
}

impl ConfigReader for ConfigFileRepo {
    fn read_config(&self) -> Result<String, ConfigError> {
        let result = read_config_file(&self.file_path);

        match result {
            Ok(file) => Ok(file),
            Err(e) => Err(ConfigError::Read(e.to_string())),
        }
    }

    fn write_config(&self, config: String) -> Result<(), ConfigError> {
        let result = save_config_file(&self.file_path, config);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ConfigError::Write(e.to_string())),
        }
    }
}

fn read_config_file(path: &Path) -> Result<String, FileError> {
    let file = std::fs::read_to_string(path);

    match file {
        Ok(file) => Ok(file.to_owned()),
        Err(e) => Err(FileError::Io(e)),
    }
}

fn save_config_file(path: &Path, content: String) -> Result<(), FileError> {
    let file = std::fs::write(path, content);

    match file {
        Ok(_) => Ok(()),
        Err(e) => Err(FileError::Io(e)),
    }
}
