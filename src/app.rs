use eframe::egui;

use crate::ui::element::Header;

pub struct App {
    version: i64,
}

impl Default for App {
    fn default() -> App {
        App { version: 1 }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Header::show(ui);
        });
    }
}
