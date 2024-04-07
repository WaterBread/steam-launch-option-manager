use std::cell::RefCell;
use std::collections::HashMap;

use eframe::egui;

struct GameConfig {
    name: String,
    launch_options: String,
    runner: String,
}

fn main() -> Result<(), eframe::Error> {
    let config_path = std::path::Path::new("./tests/config.vdf");
    let local_config_path = std::path::Path::new("./tests/localconfig.vdf");

    let config = config_file_reader::read_config_file(config_path);
    let local_config = config_file_reader::read_config_file(local_config_path);

    let local_matched = local_config.unwrap_or_else(|e| {
        println!("{:?}", e);
        std::process::exit(1);
    });

    let matched = config.unwrap_or_else(|e| {
        println!("{:?}", e);
        std::process::exit(1);
    });

    let parsed = parser::parse(&matched);
    let local_parsed = parser::local_parse(&local_matched);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let games: RefCell<HashMap<String, GameConfig>> = RefCell::new(HashMap::new());
    let parsed = parsed.unwrap();
    let local_parsed = local_parsed.unwrap();

    for (game, config) in parsed.software.valve.steam.compat_tool_mapping.iter() {
        let launch_options = local_parsed.software.valve.steam.apps.get(game);
        let launch_options = match launch_options {
            Some(app) => match &app.launch_options {
                Some(options) => options.clone(),
                None => "".to_string(),
            },
            None => "".to_string(),
        };

        games.borrow_mut().insert(
            game.clone(),
            GameConfig {
                name: config.name.clone(),
                launch_options,
                runner: config.name.clone(),
            },
        );
    }

    eframe::run_simple_native(
        "Slom (Steam Launch Option Manager)",
        options,
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Games List");

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (game, config) in games.borrow().iter() {
                        ui.horizontal(|ui| {
                            ui.label(game);
                            ui.text_edit_singleline(&mut config.launch_options.clone());
                            ui.text_edit_singleline(&mut config.runner.clone());
                        });
                    }
                });
            });
        },
    )
}
