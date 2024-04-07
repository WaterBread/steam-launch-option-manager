use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use thiserror::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct InstallConfigStore {
    #[serde(rename = "Software")]
    pub software: Software,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Software {
    #[serde(rename = "Valve")]
    pub valve: Valve,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Valve {
    #[serde(rename = "Steam")]
    pub steam: Steam,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Steam {
    #[serde(rename = "CompatToolMapping")]
    pub compat_tool_mapping: HashMap<String, GameConfig>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameConfig {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "config")]
    config: String,
    #[serde(rename = "priority")]
    priority: String,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] keyvalues_serde::Error),
}

pub fn parse(text: &str) -> Result<InstallConfigStore, ParseError> {
    let config = keyvalues_serde::from_str(&text);

    match config {
        Ok(config) => Ok(config),
        Err(e) => Err(ParseError::Serde(e)),
    }
}

pub fn serialize(config: &InstallConfigStore) -> Result<String, ParseError> {
    let config = keyvalues_serde::to_string(&config);

    match config {
        Ok(config) => Ok(config),
        Err(e) => Err(ParseError::Serde(e)),
    }
}

#[cfg(test)]
mod tests {
    use collapse::collapse;
    use collapse::collapsed_eq;

    use super::*;

    const SAMPLE: &str = r#"
    "InstallConfigStore" {
        "Software" {
            "Valve" {
                "Steam" {
                    "CompatToolMapping" {
                        "1" {
                            "config" "game1.vdf"
                            "name" "GE-Proton7-37"
                            "priority" "1"
                        }
                        "2" {
                            "config" "game2.vdf"
                            "name" "GE-Proton7-48"
                            "priority" "2"
                        }
                    }
                }
            }
        }
    }
    "#;

    #[test]
    fn test_parse() {
        let parsed = parse(SAMPLE).unwrap();

        assert_eq!(parsed.software.valve.steam.compat_tool_mapping.len(), 2);
    }

    #[test]
    fn test_serialize() {
        let parsed = parse(SAMPLE).unwrap();
        let serialized = serialize(&parsed).unwrap();

        collapsed_eq!(SAMPLE, &serialized);
    }
}
