use std::collections::HashMap;

use crate::infra::config_file_reader;

use crate::services::steamlocate_dir;
use crate::services::vdf_parser;

use crate::domain::steam_config::SteamConfig;

use crate::GameConfig;

use crate::traits::parser::Parser;
use crate::traits::steam_dir::SteamDirectory;

use anyhow::Result;

pub fn execute() -> Result<HashMap<String, GameConfig>> {
    let steam_dir_locate = steamlocate_dir::SteamLocateSteamDir {};
    let steam_dir = steam_dir_locate.get_steam_dir()?;

    let steam_config = SteamConfig::new(&steam_dir);

    let config_file_repo = config_file_reader::ConfigFileRepo {
        file_path: steam_config.config_path,
    };

    let vdf_parser = vdf_parser::parse_vdf::VdfParser::new(&config_file_repo)
        .map_err(|e| anyhow::anyhow!("Failed to parse VDF file {0}", e.to_string()))?;

    let game_runners = vdf_parser.get_game_runners()?;

    return Ok(game_runners);
}
