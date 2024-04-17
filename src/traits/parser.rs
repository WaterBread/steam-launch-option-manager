use std::collections::HashMap;
use thiserror::Error;

use crate::GameConfig;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Read error: {0}")]
    Read(String),
    #[error("Deserialization error: {0}")]
    Deserialize(String),
    #[error("Serialization error: {0}")]
    Serialize(String),
}

pub trait Parser {
    fn get_game_runners(&self) -> Result<HashMap<String, GameConfig>, ParseError>;
    fn set_game_runners(
        &mut self,
        runners: &HashMap<String, GameConfig>,
    ) -> Result<String, ParseError>;
}
