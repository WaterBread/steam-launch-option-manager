use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct InstallConfigStore {
    #[serde(rename = "Software")]
    software: Software,
}

#[derive(Deserialize, Debug)]
struct Software {
    #[serde(rename = "Valve")]
    valve: Valve,
}

#[derive(Deserialize, Debug)]
struct Valve {
    #[serde(rename = "Steam")]
    steam: Steam,
}

#[derive(Deserialize, Debug)]
struct Steam {
    #[serde(rename = "CompatToolMapping")]
    compat_tool_mapping: HashMap<String, GameConfig>,
}

#[derive(Deserialize, Debug)]
struct GameConfig {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "config")]
    config: String,
    #[serde(rename = "priority")]
    priority: String,
}

fn main() {
    // Read from file
    let file = std::fs::read_to_string("./tests/config.vdf").unwrap();

    let config: InstallConfigStore = keyvalues_serde::from_str(&file).unwrap();

    println!("{:#?}", config.software.valve.steam.compat_tool_mapping);
}
