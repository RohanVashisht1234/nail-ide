#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;

use egui_notify::Toasts;
use std::time::Duration;

// hide console window on Windows in release
use eframe::egui::{self, Color32, Label, RichText};
use egui::menu;
use egui_code_editor::{self, CodeEditor, ColorTheme, Syntax};
use egui_modal::{Icon, Modal, ModalStyle};

fn main() -> Result<(), eframe::Error> {
    let mut status = "Status:Fine";
    let mut fontsize = 15.0;
    let mut content = "Rohan Vashisht".to_string();
    let mut open_file = "".to_string();
    // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([520.0, 540.0]),
        ..Default::default()
    };
    eframe::run_simple_native("Nail IDE", options, move |ctx, _frame| {
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            if let Ok(x) = fs::read_to_string(path) {
                                content = x;
                            } else {
                                println!("Can't read file because file not of encoding utf8.")
                            }
                        } else {
                            status = "Status: No file selected";
                            println!("No file selected");
                        }
                    }
                    if ui.button("New").clicked() {
                    }
                    if ui.button("New Syntax").clicked() {
                        let mut toasts = Toasts::default();
                        toasts.info("Feature not available").set_duration(Some(Duration::from_secs(5)));
                        toasts.show(ctx);
                    }
                    if ui.button("Save").clicked() {
                        // …
                    }
                    if ui.button("Save as").clicked() {
                        // …
                    }
                    if ui.button("Close").clicked() {
                        // …
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut").clicked() {
                        // …
                    }
                    if ui.button("Copy").clicked() {
                        // …
                    }
                    if ui.button("Paste").clicked() {
                        // …
                    }
                });

                ui.menu_button("Run", |ui| {
                    if ui.button("Run file").clicked() {
                        // …
                    }
                    if ui.button("Edit Run configuration").clicked() {
                        // …
                    }
                    if ui.button("Stop running").clicked() {
                        // …
                    }
                });
            });
        });
        egui::SidePanel::left("Browse")
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .resizable(true)
            .max_width(300.0)
            .show(ctx, |ui| ui.label("text"));
        egui::CentralPanel::default().show(ctx, |ui| {
            CodeEditor::default()
                .id_source("code editor")
                .with_rows(120)
                .with_fontsize(14.0)
                .with_theme(ColorTheme::GITHUB_DARK)
                .with_syntax(Syntax::python())
                .with_numlines(true)
                .with_fontsize(fontsize)
                .show(ui, &mut content);
        });
        egui::TopBottomPanel::bottom("my_panel").show(ctx, |ui| {
            ui.label(RichText::new(status).color(Color32::LIGHT_BLUE));
        });
    })
}
