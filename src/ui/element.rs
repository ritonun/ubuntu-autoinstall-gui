use crate::state::State;

use eframe::egui;

pub fn horizontal_text_field(
    ui: &mut egui::Ui,
    label: &str,
    hover_text: &str,
    string_var: &mut String,
) {
    if hover_text.is_empty() {
        ui.horizontal(|ui| {
            ui.label(format!("{}: ", label))
                .on_hover_text(format!("{}", hover_text));
            ui.add(egui::TextEdit::singleline(string_var));
        });
    } else {
        ui.horizontal(|ui| {
            ui.label(format!("{}: ", label));
            ui.add(egui::TextEdit::singleline(string_var));
        });
    }
}

pub fn fill_vector_string(ui: &mut egui::Ui, label: &str, vec_var: &mut Vec<String>) {
    ui.horizontal(|ui| {
        ui.label(format!("{}", label));
        if ui.button("+").clicked() {
            vec_var.push(String::new());
        }
        if ui.button("-").clicked() {
            vec_var.pop();
        }
    });
    for i in 0..vec_var.len() {
        ui.horizontal(|ui| {
            ui.add_space(25.0);
            ui.label("-");
            ui.text_edit_singleline(&mut vec_var[i]);
        });
    }
}

pub struct GeneralUI;

impl GeneralUI {
    pub fn show(ui: &mut egui::Ui, state: &mut State) {
        ui.heading("Ubuntu autoinstaller.yaml generator");
        ui.label(format!("autoinstaller version: {}", state.version));

        // interactive sections
        ui.separator();
        fill_vector_string(ui, "interactive-sections", &mut state.interactive_sections);

        // locale
        ui.separator();
        horizontal_text_field(
            ui,
            "Locale",
            "Use for determining Date & Time",
            &mut state.locale,
        );

        // keyboard
        ui.separator();
        ui.label("Keyboard");
        horizontal_text_field(ui, "Layout", "", &mut state.keyboard.layout);
        horizontal_text_field(ui, "Variant", "", &mut state.keyboard.variant);
        horizontal_text_field(ui, "Toggle", "", &mut state.keyboard.toggle);

        // codecs, drivers, oem
        ui.checkbox(&mut state.codecs, "Install codecs")
            .on_hover_text("Install the ubuntu-restricted-addons package");
        ui.checkbox(&mut state.drivers, "Install drivers")
            .on_hover_text("Install available thrid-party drivers");
        ui.checkbox(&mut state.oem, "Install OEM meta-packages")
            .on_hover_text("As installing an OEM meta-package can result in installing a certain kernel, specifying both a kernel with kernel and also specifying oem.install: true may lead to an install failure due to conflicting kernel requirements. When using oem.install, it is recommended to not specify a kernel.");

        // Show errors
        ui.separator();
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
