use crate::state::State;
use crate::validators::*;
use eframe::egui;

pub struct GeneralUI;

impl GeneralUI {
    pub fn show(ui: &mut egui::Ui, state: &mut State) {
        ui.heading("Ubuntu autoinstaller.yaml generator");
        ui.label(format!("autoinstaller version: {}", state.version));
        ui.label("interactive-sections"); // todo

        // locale
        ui.horizontal(|ui| {
            ui.label("Locale: ");
            let locale_ui = ui.add(egui::TextEdit::singleline(&mut state.locale));
            if locale_ui.lost_focus() {
                if let Err(e) = validate_locale(&state.locale) {
                    state.errors.push(e);
                }
            }
        });

        // keyboard

        // Show errors
        ui.label("Errors: ");
        if !state.errors.is_empty() {
            for error in &state.errors {
                ui.colored_label(egui::Color32::RED, error);
            }
        } else {
            ui.colored_label(egui::Color32::GREEN, "All field are OK");
        }
    }
}
