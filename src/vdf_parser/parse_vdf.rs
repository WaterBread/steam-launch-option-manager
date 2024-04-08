use std::collections::HashMap;

use keyvalues_parser::{Obj, Vdf};

use crate::{
    traits::parser::{ParseError, Parser},
    GameConfig,
};

struct VdfParser<'a> {
    vdf: keyvalues_parser::Vdf<'a>,
}

impl VdfParser<'_> {
    fn new(vdf_string: &str) -> Option<VdfParser<'_>> {
        let vdf = Vdf::parse(vdf_string).ok()?;
        Some(VdfParser { vdf })
    }
}

impl Parser for VdfParser<'_> {
    fn get_game_runners(
        &self,
        vdf_string: &str,
    ) -> Result<HashMap<String, GameConfig>, ParseError> {
        let get_mapping_attr = || -> Option<&Obj> {
            Some(
                self.vdf
                    .value
                    .get_obj()?
                    .get("Software")?
                    .get(0)?
                    .get_obj()?
                    .get("Valve")?
                    .get(0)?
                    .get_obj()?
                    .get("Steam")?
                    .get(0)?
                    .get_obj()?
                    .get("CompatToolMapping")?
                    .get(0)?
                    .get_obj()?,
            )
        };

        let get_game_runners =
            |mapping_attr: &Obj<'static>| -> Option<HashMap<String, GameConfig>> {
                let mut game_runners: HashMap<String, GameConfig> = HashMap::new();

                for (key, value) in mapping_attr.iter() {
                    let value = value.get(0)?;

                    let config_path = value.get_obj()?.get("name")?.get(0)?.get_str()?;

                    game_runners.insert(
                        key.to_string(),
                        GameConfig {
                            name: key.to_string(),
                            launch_options: String::new(),
                            runner: config_path.to_string(),
                        },
                    );
                }

                Some(game_runners)
            };

        let mapping_attr =
            get_mapping_attr().ok_or(ParseError::Deserialize("Error".to_string()))?;

        let game_runners =
            get_game_runners(mapping_attr).ok_or(ParseError::Deserialize("Error".to_string()))?;

        Ok(game_runners)
    }

    fn set_game_runners(
        &mut self,
        runners: HashMap<String, GameConfig>,
    ) -> Result<&str, ParseError> {
        let mut get_attribute_mut = |app_id: &str| -> Option<&mut String> {
            let name = self
                .vdf
                .value
                .get_mut_obj()?
                .get_mut("Software")?
                .get_mut(0)?
                .get_mut_obj()?
                .get_mut("Valve")?
                .get_mut(0)?
                .get_mut_obj()?
                .get_mut("Steam")?
                .get_mut(0)?
                .get_mut_obj()?
                .get_mut("CompatToolMapping")?
                .get_mut(0)?
                .get_mut_obj()?
                .get_mut(app_id)?
                .get_mut(0)?
                .get_mut_obj()?
                .get_mut("name")?
                .get_mut(0)?
                .get_mut_str()?
                .to_mut();

            Some(name)
        };

        for (app_id, game_config) in runners.iter() {
            let mut attr =
                get_attribute_mut(app_id).ok_or(ParseError::Serialize("Error".to_string()))?;

            *attr = game_config.runner;
        }

        Ok("Success")
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_game_runners() {
        let vdf = r#"
            "Software"
            {
                "Valve"
                {
                    "Steam"
                    {
                        "CompatToolMapping"
                        {
                          "0"
                          {
                            "name"		"proton_experimental"
                            "config"		""
                            "priority"		"75"
                          }
                          "509980"
                          {
                            "name"		"proton_63"
                            "config"		""
                            "priority"		"250"
                          }
                        }
                    }
                }
            }
        "#;

        let parser = VdfParser::new(vdf).unwrap();
        let game_runners = parser.get_game_runners(vdf).unwrap();

        assert_eq!(game_runners.len(), 2);
        assert_eq!(game_runners.get("1").unwrap().runner, "runner1");
        assert_eq!(game_runners.get("2").unwrap().runner, "runner2");
    }

    #[test]
    fn test_set_game_runners() {
        let vdf = r#"
            "Software"
            {
                "Valve"
                {
                    "Steam"
                    {
                        "CompatToolMapping"
                        {
                            "1"    "name"    "runner1"
                            "2"    "name"    "runner2"
                        }
                    }
                }
            }
        "#;

        let mut parser = VdfParser::new(vdf).unwrap();
        let mut game_runners = parser.get_game_runners(vdf).unwrap();

        game_runners.insert(
            "3".to_string(),
            GameConfig {
                name: "3".to_string(),
                launch_options: String::new(),
                runner: "runner3".to_string(),
            },
        );

        parser.set_game_runners(game_runners).unwrap();

        let game_runners = parser.get_game_runners(vdf).unwrap();

        assert_eq!(game_runners.len(), 3);
        assert_eq!(game_runners.get("1").unwrap().runner, "runner1");
        assert_eq!(game_runners.get("2").unwrap().runner, "runner2");
        assert_eq!(game_runners.get("3").unwrap().runner, "runner3");
    }
}
