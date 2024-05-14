mod crypt;

use eframe::egui;

#[derive(Default)]
struct MyGui {
    input_file_path: Option<String>
}

impl eframe::App for MyGui {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Select your file to be ciphered or deciphered: ");

            if ui.button("Browse").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.input_file_path = Some(path.display().to_string());
                }
            }

            if self.input_file_path.is_some() {
                ui.label(format!("Choosed file: {}", self.input_file_path.clone().unwrap()));
            }

            if ui.button("Execute!").clicked() {
                crypt::start_crypt(self.input_file_path.clone().unwrap());
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500., 100.])
            .with_drag_and_drop(false),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Ciphering/Deciphering on demand!",
        options,
        Box::new(|_cc| Box::<MyGui>::default())
    );
}
