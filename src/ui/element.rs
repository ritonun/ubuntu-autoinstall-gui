use crate::state::State;

use eframe::egui;

pub fn horizontal_text_field(ui: &mut egui::Ui, label: &str, string_var: &mut String) {
    ui.horizontal(|ui| {
        ui.label(format!("{}: ", label));
        ui.add(egui::TextEdit::singleline(string_var));
    });
}

pub struct GeneralUI;

impl GeneralUI {
    pub fn show(ui: &mut egui::Ui, state: &mut State) {
        ui.heading("Ubuntu autoinstaller.yaml generator");
        ui.label(format!("autoinstaller version: {}", state.version));
        ui.label("interactive-sections"); // todo

        // locale
        horizontal_text_field(ui, "Locale", &mut state.locale);

        // keyboard
        ui.separator();
        ui.label("Keyboard");
        horizontal_text_field(ui, "Keyboard layout", &mut state.keyboard.layout);

        // Show errors
        if ui.button("Validate field").clicked() {
            state.validate_fields();
        }
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
