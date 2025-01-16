// src/ui.rs
use eframe::egui;
use crate::app::CalculatorApp;

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let button_size = egui::vec2(60.0, 40.0);
            
            ui.add_space(10.0);
            ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
            
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                ui.add(egui::Label::new(&self.display)
                    .wrap(false)
                    .sense(egui::Sense::click()))
                    .on_hover_text("Anzeige");
            });
            
            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.rad_mode, true, "RAD");
                ui.selectable_value(&mut self.rad_mode, false, "DEG");
            });

            ui.add_space(10.0);
            
            egui::Grid::new("scientific_buttons")
                .spacing(egui::vec2(5.0, 5.0))
                .show(ui, |ui| {
                    for btn in ["sin", "cos", "tan", "ln"].iter() {
                        if ui.add_sized(button_size, egui::Button::new(*btn)).clicked() {
                            self.handle_button(btn);
                        }
                    }
                    ui.end_row();

                    for btn in ["asin", "acos", "atan", "log"].iter() {
                        if ui.add_sized(button_size, egui::Button::new(*btn)).clicked() {
                            self.handle_button(btn);
                        }
                    }
                    ui.end_row();
                });

            ui.add_space(10.0);

            egui::Grid::new("basic_buttons")
                .spacing(egui::vec2(5.0, 5.0))
                .show(ui, |ui| {
                    for btn in ["C", "±", "π", "/"].iter() {
                        if ui.add_sized(button_size, egui::Button::new(*btn)).clicked() {
                            self.handle_button(btn);
                        }
                    }
                    ui.end_row();

                    for row in [
                        ["7", "8", "9", "*"],
                        ["4", "5", "6", "-"],
                        ["1", "2", "3", "+"],
                        ["0", ".", "e", "="],
                    ].iter() {
                        for btn in row.iter() {
                            if ui.add_sized(button_size, egui::Button::new(*btn)).clicked() {
                                self.handle_button(btn);
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    }
}

