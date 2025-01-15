use eframe::egui;
use std::f64::consts::{PI, E};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 600.0)),
        resizable: true,
        ..Default::default()
    };
    
    eframe::run_native(
        "Wissenschaftlicher Taschenrechner",
        options,
        Box::new(|_cc| Box::new(CalculatorApp::default())),
    )
}

struct CalculatorApp {
    display: String,
    current_input: String,
    last_result: f64,
    rad_mode: bool,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            display: String::from("0"),
            current_input: String::new(),
            last_result: 0.0,
            rad_mode: true,
        }
    }
}

impl CalculatorApp {
    fn format_expression(&self, expr: &str) -> String {
        let mut formatted = String::new();
        let mut prev_char_is_digit = false;

        for c in expr.chars() {
            match c {
                '0'..='9' | '.' => {
                    if !prev_char_is_digit && !formatted.is_empty() {
                        formatted.push(' ');
                    }
                    formatted.push(c);
                    prev_char_is_digit = true;
                }
                '+' | '-' | '*' | '/' => {
                    if prev_char_is_digit {
                        formatted.push(' ');
                    }
                    formatted.push(c);
                    formatted.push(' ');
                    prev_char_is_digit = false;
                }
                _ => {}
            }
        }
        formatted.trim().to_string()
    }

    fn evaluate(&self, expr: &str) -> Result<f64, String> {
        // Format the expression first with proper spacing
        let formatted_expr = self.format_expression(expr);
        println!("Formatted expression: {}", formatted_expr); // Debug log
        
        // Split formatted expression into tokens
        let tokens: Vec<&str> = formatted_expr.split_whitespace().collect();
        println!("Tokens: {:?}", tokens); // Debug log
        
        // Need at least one number
        if tokens.is_empty() {
            return Err("Empty expression".to_string());
        }

        // If single number, parse and return it
        if tokens.len() == 1 {
            return tokens[0].parse::<f64>()
                .map_err(|_| "Invalid number".to_string());
        }

        // Need three tokens for operation: number operator number
        if tokens.len() != 3 {
            return Err("Invalid expression format".to_string());
        }

        let left = tokens[0].parse::<f64>()
            .map_err(|_| "Invalid first number".to_string())?;
        let operator = tokens[1];
        let right = tokens[2].parse::<f64>()
            .map_err(|_| "Invalid second number".to_string())?;

        match operator {
            "+" => Ok(left + right),
            "-" => Ok(left - right),
            "*" => Ok(left * right),
            "/" => {
                if right == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(left / right)
                }
            },
            _ => Err("Invalid operator".to_string())
        }
    }

    fn handle_button(&mut self, text: &str) {
        match text {
            "C" => {
                self.current_input.clear();
                self.display = "0".to_string();
            },
            "=" => {
                let input = self.current_input.clone();
                println!("Evaluating input: {}", input); // Debug logging
                
                match self.evaluate(&input) {
                    Ok(result) => {
                        self.last_result = result;
                        self.display = format!("{:.10}", result)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string();
                        self.current_input = self.display.clone();
                        println!("Calculation result: {}", self.display); // Debug logging
                    },
                    Err(e) => {
                        println!("Calculation error: {:?}", e); // Debug logging
                        self.display = "Error".to_string();
                        self.current_input.clear();
                    }
                }
            },
            "±" => {
                if let Ok(mut num) = self.current_input.parse::<f64>() {
                    num = -num;
                    self.current_input = num.to_string();
                    self.display = self.current_input.clone();
                }
            },
            "π" => {
                self.current_input = PI.to_string();
                self.display = "π".to_string();
            },
            "e" => {
                self.current_input = E.to_string();
                self.display = "e".to_string();
            },
            _ => {
                if self.current_input == "0" {
                    self.current_input = text.to_string();
                } else {
                    self.current_input.push_str(text);
                }
                self.display = self.current_input.clone();
            }
        }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Stil-Definitionen
            let button_size = egui::vec2(60.0, 40.0);
            
            // Display-Bereich
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

            // Modus-Umschalter (RAD/DEG)
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.rad_mode, true, "RAD");
                ui.selectable_value(&mut self.rad_mode, false, "DEG");
            });

            // Tastenbereich
            ui.add_space(10.0);
            
            // Wissenschaftliche Funktionen
            egui::Grid::new("scientific_buttons")
                .spacing(egui::vec2(5.0, 5.0))
                .show(ui, |ui| {
                    // Erste Reihe
                    for btn in ["sin", "cos", "tan", "ln"].iter() {
                        if ui.add_sized(button_size, egui::Button::new(*btn)).clicked() {
                            self.handle_button(btn);
                        }
                    }
                    ui.end_row();

                    // Zweite Reihe
                    for btn in ["asin", "acos", "atan", "log"].iter() {
                        if ui.add_sized(button_size, egui::Button::new(*btn)).clicked() {
                            self.handle_button(btn);
                        }
                    }
                    ui.end_row();
                });

            ui.add_space(10.0);

            // Grundlegende Funktionen
            egui::Grid::new("basic_buttons")
                .spacing(egui::vec2(5.0, 5.0))
                .show(ui, |ui| {
                    // Erste Reihe
                    for btn in ["C", "±", "π", "/"].iter() {
                        if ui.add_sized(button_size, egui::Button::new(*btn)).clicked() {
                            self.handle_button(btn);
                        }
                    }
                    ui.end_row();

                    // Weitere Reihen
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
