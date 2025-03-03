use crate::state::State;

use eframe::egui;

const OFFSET: f32 = 25.0;

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
            ui.add_space(OFFSET);
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

        // early commands
        ui.separator();
        fill_vector_string(ui, "early commands", &mut state.early_commands);

        // locale
        ui.separator();
        horizontal_text_field(
            ui,
            "Locale",
            "Use for determining Date & Time",
            &mut state.locale,
        );

        // refresh installer
        ui.separator();
        ui.label("Refresh installer")
            .on_hover_text("Installer update to a new version in givenn channel");
        ui.horizontal(|ui| {
            ui.add_space(OFFSET);
            ui.checkbox(&mut state.refresh_installer.update, "Update");
        });
        horizontal_text_field(
            ui,
            "Channel",
            "Channel to check for updates",
            &mut state.refresh_installer.channel,
        );

        // keyboard
        ui.separator();
        ui.label("Keyboard");
        horizontal_text_field(ui, "Layout", "", &mut state.keyboard.layout);
        horizontal_text_field(ui, "Variant", "", &mut state.keyboard.variant);
        horizontal_text_field(ui, "Toggle", "", &mut state.keyboard.toggle);

        // source
        ui.separator();
        ui.label("Source")
            .on_hover_text("Installer searches for available third-party drivers");
        ui.horizontal(|ui| {
            ui.add_space(OFFSET);
            ui.checkbox(&mut state.source.search_drivers, "Search drivers");
        });
        horizontal_text_field(
            ui,
            "Id",
            "Identifier of the source to install",
            &mut state.source.id,
        );

        // proxy
        ui.separator();
        horizontal_text_field(
            ui,
            "Proxy",
            "Proxy to configure during installation",
            &mut state.proxy,
        );

        // identity
        ui.separator();
        ui.label("Identity");
        horizontal_text_field(
            ui,
            "Real name",
            "Optionnal field",
            &mut state.identity.realname,
        );
        horizontal_text_field(
            ui,
            "User name",
            "User name to create",
            &mut state.identity.username,
        );
        horizontal_text_field(
            ui,
            "Hostname",
            "Hostname for the system",
            &mut state.identity.hostname,
        );
        horizontal_text_field(
            ui,
            "Password HASHED",
            "Hashed password, please see documentation",
            &mut state.identity.password_hash,
        );

        // ubuntu pro token
        ui.separator();
        ui.label("Ubuntu Pro");
        horizontal_text_field(
            ui,
            "Ubuntu pro token",
            "A contract token to attach to an existing Ubuntu Pro subscription",
            &mut state.ubuntu_pro_token,
        );

        // ssh
        ui.separator();
        ui.label("SSH");
        ui.horizontal(|ui| {
            ui.add_space(OFFSET);
            ui.checkbox(&mut state.ssh.install_server, "Search drivers");
        });
        fill_vector_string(ui, "Autorized keys", &mut state.ssh.authorized_keys);
        ui.horizontal(|ui| {
            ui.add_space(OFFSET);
            ui.checkbox(&mut state.ssh.allow_pw, "Allow pw");
        });
        if state.ssh.authorized_keys.is_empty() {
            state.ssh.allow_pw = false;
        } else {
            state.ssh.allow_pw = true;
        }

        // codecs, drivers, oem
        ui.separator();
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
