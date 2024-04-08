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

fn bogged<'a, 'b>(vdf: &'a VdfParser<'b>) -> Option<&'a Obj<'b>>
where
    'a: 'b,
{
    vdf.vdf
        .value
        .get_obj()?
        .get("Software")?
        .first()?
        .get_obj()?
        .get("Valve")?
        .first()?
        .get_obj()?
        .get("Steam")?
        .first()?
        .get_obj()?
        .get("CompatToolMapping")?
        .first()?
        .get_obj()
}

fn bogged_2(ob: &Obj) -> Option<HashMap<String, GameConfig>> {
    let mut game_runners: HashMap<String, GameConfig> = HashMap::new();

    for (key, value) in ob.iter() {
        let value = value.first()?;

        let config_path = value.get_obj()?.get("name")?.first()?.get_str()?;

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
}

fn bogged_3<'a, 'b>(
    vdf: &'a mut keyvalues_parser::Vdf<'b>,
    app_id: &'a str,
) -> Option<&'a mut String>
where
    'b: 'a,
{
    vdf.value
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
        .get_mut_str()
        .map(std::borrow::Cow::to_mut)
}

impl Parser for VdfParser<'_> {
    fn get_game_runners(&self) -> Result<HashMap<String, GameConfig>, ParseError> {
        bogged(self)
            .and_then(|o| bogged_2(o))
            .ok_or(ParseError::Deserialize("Error".to_string()))
    }

    fn set_game_runners(
        &mut self,
        runners: HashMap<String, GameConfig>,
    ) -> Result<&str, ParseError> {
        for (app_id, game_config) in runners {
            let attr = bogged_3(&mut self.vdf, &app_id)
                .ok_or(ParseError::Deserialize("Error".to_string()))?;

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
        "InstallConfigStore"
        {
            "Software"
            {
                "Valve"
                {
                    "Steam"
                    {
                        "AutoUpdateWindowEnabled"		"0"
                        "ipv6check_http_state"		"good"
                        "ipv6check_udp_state"		"good"
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
                            "1549970"
                            {
                                "name"		"proton_63"
                                "config"		""
                                "priority"		"250"
                            }

                        }
                        "RecentDownloadRate"		"1004281"
                        "LastConfigstoreUploadTime"		"1712429641"
                    }
                }
            }
        }
        "#;

        let parser = VdfParser::new(vdf).unwrap();
        let game_runners = parser.get_game_runners().unwrap();

        assert_eq!(game_runners.len(), 3);
        assert_eq!(game_runners.get("0").unwrap().runner, "proton_experimental");
        assert_eq!(game_runners.get("509980").unwrap().runner, "proton_63");
    }

    #[test]
    fn test_set_game_runners() {
        let vdf = r#"
        "InstallConfigStore"
        {
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
        }
        "#;

        let mut parser = VdfParser::new(vdf).unwrap();
        let mut game_runners = parser.get_game_runners().unwrap();

        game_runners.insert(
            "509980".to_string(),
            GameConfig {
                name: "509980".to_string(),
                launch_options: String::new(),
                runner: "proton_lmao".to_string(),
            },
        );

        parser.set_game_runners(game_runners).unwrap();

        let game_runners = parser.get_game_runners().unwrap();

        assert_eq!(game_runners.len(), 2);
        assert_eq!(game_runners.get("0").unwrap().runner, "proton_experimental");
        assert_eq!(game_runners.get("509980").unwrap().runner, "proton_lmao");
    }
}
