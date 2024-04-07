use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use thiserror::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct UserLocalConfigStore {
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
    #[serde(rename = "apps")]
    pub apps: HashMap<String, AppConfig>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppConfig {
    #[serde(
        rename = "LastPlayed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    last_played: Option<String>,
    #[serde(rename = "Playtime", default, skip_serializing_if = "Option::is_none")]
    playtime: Option<String>,
    #[serde(
        rename = "LaunchOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_options: Option<String>,
    #[serde(rename = "cloud", default, skip_serializing_if = "Option::is_none")]
    cloud: Option<CloudConfig>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CloudConfig {
    #[serde(rename = "last_sync_state")]
    last_sync_state: String,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] keyvalues_serde::Error),
}

pub fn parse(text: &str) -> Result<UserLocalConfigStore, ParseError> {
    let config = keyvalues_serde::from_str(&text);

    match config {
        Ok(config) => Ok(config),
        Err(e) => Err(ParseError::Serde(e)),
    }
}

pub fn serialize(config: &UserLocalConfigStore) -> Result<String, ParseError> {
    let config = keyvalues_serde::to_string(&config);

    match config {
        Ok(config) => Ok(config),
        Err(e) => Err(ParseError::Serde(e)),
    }
}
