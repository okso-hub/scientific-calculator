use eframe::egui;
use std::f64::consts::{PI, E};

pub struct CalculatorApp {
    pub display: String,
    pub current_input: String,
    pub last_result: f64,
    pub rad_mode: bool,
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

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Operator(char),
}

impl CalculatorApp {
    fn get_precedence(op: char) -> u8 {
        match op {
            '+' | '-' => 1,
            '*' | '/' => 2,
            _ => 0,
        }
    }

    fn infix_to_postfix(&self, tokens: Vec<&str>) -> Result<Vec<Token>, String> {
        let mut output = Vec::new();
        let mut operator_stack = Vec::new();

        for token in tokens {
            match token {
                "+" | "-" | "*" | "/" => {
                    let op = token.chars().next().unwrap();
                    while let Some(&top) = operator_stack.last() {
                        if Self::get_precedence(top) >= Self::get_precedence(op) {
                            output.push(Token::Operator(operator_stack.pop().unwrap()));
                        } else {
                            break;
                        }
                    }
                    operator_stack.push(op);
                }
                num => {
                    let number = num.parse::<f64>()
                        .map_err(|_| "Invalid number".to_string())?;
                    output.push(Token::Number(number));
                }
            }
        }

        while let Some(op) = operator_stack.pop() {
            output.push(Token::Operator(op));
        }

        Ok(output)
    }

    fn evaluate_postfix(&self, postfix: Vec<Token>) -> Result<f64, String> {
        let mut stack = Vec::new();

        for token in postfix {
            match token {
                Token::Number(num) => stack.push(num),
                Token::Operator(op) => {
                    let b = stack.pop().ok_or("Invalid expression")?;
                    let a = stack.pop().ok_or("Invalid expression")?;
                    let result = match op {
                        '+' => a + b,
                        '-' => a - b,
                        '*' => a * b,
                        '/' => {
                            if b == 0.0 {
                                return Err("Division by zero".to_string());
                            }
                            a / b
                        }
                        _ => return Err("Invalid operator".to_string()),
                    };
                    stack.push(result);
                }
            }
        }

        stack.pop().ok_or("Invalid expression".to_string())
    }

    fn format_expression(&self, expr: &str) -> String {
        let mut formatted = String::new();
        let mut prev_char_is_digit = false;

        for c in expr.chars() {
            match c {
                '0'..='9' | '.' => {
                    if !prev_char_is_digit && !formatted.is_empty() && !formatted.ends_with(' ') {
                        formatted.push(' ');
                    }
                    formatted.push(c);
                    prev_char_is_digit = true;
                }
                '+' | '-' | '*' | '/' => {
                    if prev_char_is_digit && !formatted.ends_with(' ') {
                        formatted.push(' ');
                    }
                    formatted.push(c);
                    if !formatted.ends_with(' ') {
                        formatted.push(' ');
                    }
                    prev_char_is_digit = false;
                }
                _ => {}
            }
        }
        formatted.trim().to_string()
    }

    fn evaluate(&self, expr: &str) -> Result<f64, String> {
        let formatted_expr = self.format_expression(expr);
        println!("Formatted expression: {}", formatted_expr);
        
        let tokens: Vec<&str> = formatted_expr.split_whitespace().collect();
        println!("Tokens: {:?}", tokens);
        
        if tokens.is_empty() {
            return Err("Empty expression".to_string());
        }

        let postfix = self.infix_to_postfix(tokens)?;
        self.evaluate_postfix(postfix)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_initialization() {
        let calc = CalculatorApp::default();
        assert_eq!(calc.display, "0");
        assert_eq!(calc.current_input, "");
        assert_eq!(calc.last_result, 0.0);
    }

    #[test]
    fn test_basic_arithmetic() {
        let calc = CalculatorApp::default();
        
        assert_eq!(calc.evaluate("5+3").unwrap(), 8.0);
        assert_eq!(calc.evaluate("10-4").unwrap(), 6.0);
        assert_eq!(calc.evaluate("8*8").unwrap(), 64.0);
        assert_eq!(calc.evaluate("15/3").unwrap(), 5.0);
    }

    #[test]
    fn test_format_expression() {
        let calc = CalculatorApp::default();
        
        assert_eq!(calc.format_expression("5+3"), "5 + 3");
        assert_eq!(calc.format_expression("10*5"), "10 * 5");
        assert_eq!(calc.format_expression("8-2"), "8 - 2");
    }

    #[test]
    fn test_error_handling() {
        let calc = CalculatorApp::default();
        
        assert!(calc.evaluate("").is_err());
        assert!(calc.evaluate("abc").is_err());
        assert!(calc.evaluate("5/0").is_err());
        // Removed test for "1+2+3" as it's now valid
    }

    #[test]
    fn test_multiple_operations() {
        let calc = CalculatorApp::default();
        
        assert_eq!(calc.evaluate("1+2+3").unwrap(), 6.0);
        assert_eq!(calc.evaluate("10-2-3").unwrap(), 5.0);
        assert_eq!(calc.evaluate("2*3*4").unwrap(), 24.0);
        assert_eq!(calc.evaluate("24/2/3").unwrap(), 4.0);
        assert_eq!(calc.evaluate("1+2*3").unwrap(), 7.0);  // Updated: 2*3 = 6, then 1+6 = 7
    }

    #[test]
    fn test_operator_precedence() {
        let calc = CalculatorApp::default();
        assert_eq!(calc.evaluate("32-6*6").unwrap(), -4.0);
        assert_eq!(calc.evaluate("2+3*4").unwrap(), 14.0);
        assert_eq!(calc.evaluate("10/2*5").unwrap(), 25.0);
        assert_eq!(calc.evaluate("2*3+4*5").unwrap(), 26.0);
    }
}