use std::path::PathBuf;

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
        let file = std::fs::read_to_string(&self.file_path);

        match file {
            Ok(file) => Ok(file.to_owned()),
            Err(e) => Err(ConfigError::Read(e.to_string())),
        }
    }

    fn write_config(&self, config: String) -> Result<(), ConfigError> {
        let file = std::fs::write("./test.vdf", config);

        match file {
            Ok(_) => Ok(()),
            Err(e) => Err(ConfigError::Write(e.to_string())),
        }
    }
}
