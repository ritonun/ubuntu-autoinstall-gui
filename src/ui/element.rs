use eframe::egui;

pub struct Header;

impl Header {
    pub fn show(ui: &mut egui::Ui) {
        ui.heading("Ubuntu autoinstaller.yaml generator");
    }
}
