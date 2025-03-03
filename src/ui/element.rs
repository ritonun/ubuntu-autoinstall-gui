use crate::state::State;
use eframe::egui;

pub struct GeneralUI;

impl GeneralUI {
    pub fn show(ui: &mut egui::Ui, state: &mut State) {
        ui.heading("Ubuntu autoinstaller.yaml generator");
        ui.label(format!("version: {}", state.version));
        ui.label("interactive-sections");
        let locale_ui = ui.add(egui::TextEdit::singleline(&mut state.locale));

        if locale_ui.lost_focus() {
            println!("{:?}", state);
        }
    }
}
