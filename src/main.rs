mod app;
mod ui;

use app::App;
use eframe::NativeOptions;

fn main() {
    let options = NativeOptions::default();
    match eframe::run_native(
        "My Egui App",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    ) {
        Ok(_) => {}
        Err(e) => panic!("Error when running eframe: {}", e),
    }
}
