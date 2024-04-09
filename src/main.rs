pub mod traits;

mod apps;
mod domain;
mod infra;
mod services;

use std::path::Path;

use eframe::egui;
use traits::parser::Parser;

pub struct GameConfig {
    name: String,
    launch_options: String,
    runner: String,
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let mut game_runners = apps::get_game_launch_configs::execute().unwrap();

    eframe::run_simple_native(
        "Slom (Steam Launch Option Manager)",
        options,
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Games List");

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (game, config) in game_runners.iter_mut() {
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
