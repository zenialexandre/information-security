mod constants;
mod rsa_keys;
mod crypt;

use core::panic;
use std::fs::{read_dir, remove_file};
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
                let _ = rsa_keys::start_rsa_keys();
                let _ = crypt::start_crypt(self.input_file_path.clone().unwrap()).unwrap();
            }

            if ctx.input(|_ctx| _ctx.viewport().close_requested()) {
                let dir_entires = match read_dir(constants::path::USER_FILE_PATH_PLACEHOLDER) {
                    Ok(entries) => entries,
                    Err(error) => panic!("{}", error)
                };

                for entry in dir_entires {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(error) => panic!("{}", error)
                    };

                    let _ = remove_file(entry.path());
                }

                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400., 150.])
            .with_drag_and_drop(false),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Ciphering/Deciphering on demand with RSA!",
        options,
        Box::new(|_cc| Box::<MyGui>::default())
    );
}
