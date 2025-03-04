mod app;
mod generator_yaml;
mod state;
mod ui;
mod utils;
mod validators;

use app::App;
use eframe::NativeOptions;

fn main() {
    let options = NativeOptions::default();
    match eframe::run_native(
        "Ubuntu autoinstaller.yaml",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    ) {
        Ok(_) => {}
        Err(e) => panic!("Error when running eframe: {}", e),
    }
}
