use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config: {0}")]
    Read(String),
    #[error("Failed to write config: {0}")]
    Write(String),
}

pub trait ConfigReader {
    fn read_config(&self) -> Result<String, ConfigError>;
    fn write_config(&self, config: String) -> Result<(), ConfigError>;
}
