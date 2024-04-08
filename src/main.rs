pub mod traits;

mod vdf_parser;

use std::collections::HashMap;
use std::path::Path;

use eframe::egui;
struct GameConfig {
    name: String,
    launch_options: String,
    runner: String,
}

fn main() -> Result<(), eframe::Error> {
    let mut games_hash_map: HashMap<String, GameConfig> = HashMap::new();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_simple_native(
        "Slom (Steam Launch Option Manager)",
        options,
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Games List");

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (game, config) in games_hash_map.iter_mut() {
                        ui.horizontal(|ui| {
                            ui.label(game);
                            ui.text_edit_singleline(&mut config.runner);
                        });
                    }
                });

                // ui.horizontal(|ui| {
                //     if ui.button("Save").clicked() {
                //         for (game, config) in games_hash_map.iter() {
                //             parser::set_game_runner(game, config.runner.clone(), &mut vdf);
                //         }

                //         config_file_reader::save_config_file(
                //             Path::new("./test.vdf"),
                //             String::from(&vdf.to_string()),
                //         );
                //     }
                // });
            });
        },
    )
}
