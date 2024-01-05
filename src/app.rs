use std::fs::File;
use std::io::prelude::*;

use eframe::egui::{ScrollArea};
use crate::problem;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)]
    problem: problem::Problem,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            problem: Default::default(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("AOC2023 Day 1");

            ui.separator();

            ui.label("Input Block");
            if ui.button("Open File").clicked() {
                let open_file: String;
                let file_result: Result<(), String> = match tinyfiledialogs::open_file_dialog("Open", "", None) {
                    None => {
                        open_file = "null".to_string();
                        Err("No file provided".to_string())
                    }
                    Some(file) => {
                        open_file = file.clone();
                        let mut handle = File::open(file).expect("Could not open file");
                        self.label.clear();
                        handle.read_to_string(&mut self.label).expect("Could not read from file");
                        Ok(())
                    }
                };
                println!("Opening File: {}", open_file);
            }
            if ui.button("Clear Input").clicked() {
                self.label.clear();
            }
            ScrollArea::vertical().id_source("Input").max_height(0.5f32)
                .show(ui, |ui| {
                    ui.text_edit_multiline(&mut self.label);
                });

            ui.separator();

            ui.label("Control Block");

            if ui.button("Process Part 1").clicked() {
                self.problem.set_input(&self.label);
                self.problem.process_as_part_1();
            }
            
            if ui.button("Process Part 2").clicked() {
                self.problem.set_input(&self.label);
                self.problem.process_as_part_2();
            }

            ui.separator();

            ui.label("State Block");
            ScrollArea::vertical().id_source("State").max_height(0.5f32)
                .show(ui, |ui| {
                    ui.text_edit_multiline(&mut self.problem.log);
                });

            ui.separator();

            ui.label("Answer: ");
            if let Some(answer) = self.problem.part_1_answer {
                ui.label(format!("Part 1: {}", answer));
            }
            if let Some(answer) = self.problem.part_2_answer {
                ui.label(format!("Part 2: {}", answer));
            }

        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
