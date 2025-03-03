use eframe::egui;

use crate::state::State;
use crate::ui::element::GeneralUI;

pub struct App {
    state: State,
}

impl Default for App {
    fn default() -> App {
        App {
            state: State::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            GeneralUI::show(ui, &mut self.state);
        });
    }
}
