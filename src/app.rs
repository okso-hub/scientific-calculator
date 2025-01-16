// src/app.rs
use std::f64::consts::{PI, E};
use crate::calculator::Calculator;

pub struct CalculatorApp {
    pub display: String,
    pub current_input: String,
    pub last_result: f64,
    pub rad_mode: bool,
    pub calculator: Calculator,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            display: String::from("0"),
            current_input: String::new(),
            last_result: 0.0,
            rad_mode: true,
            calculator: Calculator::new(true),
        }
    }
}

impl CalculatorApp {
    pub fn handle_button(&mut self, text: &str) {
        match text {
            "C" => {
                self.current_input.clear();
                self.display = "0".to_string();
            },
            "=" => {
                let input = self.current_input.clone();
                println!("Evaluating input: {}", input);
                
                match self.calculator.evaluate(&input) {
                    Ok(result) => {
                        self.last_result = result;
                        self.display = format!("{:.10}", result)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string();
                        self.current_input = self.display.clone();
                        println!("Calculation result: {}", self.display);
                    },
                    Err(e) => {
                        println!("Calculation error: {:?}", e);
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

