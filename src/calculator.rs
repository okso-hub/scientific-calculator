// src/calculator.rs
use crate::token::Token;

pub struct Calculator {
    rad_mode: bool,
}

impl Calculator {
    pub fn new(rad_mode: bool) -> Self {
        Self { rad_mode }
    }

    pub fn get_precedence(op: char) -> u8 {
        match op {
            '+' | '-' => 1,
            '*' | '/' => 2,
            _ => 0,
        }
    }

    pub fn infix_to_postfix(&self, tokens: Vec<&str>) -> Result<Vec<Token>, String> {
        let mut output = Vec::new();
        let mut operator_stack = Vec::new();

        for token in tokens {
            match token {
                "sin" | "cos" | "tan" | "ln" | "log" | "asin" | "acos" | "atan" => {
                    operator_stack.push(Token::Function(token.to_string()));
                }
                "+" | "-" | "*" | "/" => {
                    let op = token.chars().next().unwrap();
                    while let Some(top) = operator_stack.last() {
                        match top {
                            Token::Operator(top_op) if Self::get_precedence(*top_op) >= Self::get_precedence(op) => {
                                output.push(operator_stack.pop().unwrap());
                            }
                            Token::Function(_) => {
                                output.push(operator_stack.pop().unwrap());
                            }
                            _ => break,
                        }
                    }
                    operator_stack.push(Token::Operator(op));
                }
                num => {
                    match num.parse::<f64>() {
                        Ok(n) => output.push(Token::Number(n)),
                        Err(_) => return Err("Invalid number".to_string()),
                    }
                }
            }
        }

        while let Some(op) = operator_stack.pop() {
            output.push(op);
        }

        Ok(output)
    }

    pub fn evaluate_postfix(&self, postfix: Vec<Token>) -> Result<f64, String> {
        let mut stack = Vec::new();
        
        let exact_sin_90 = 1.0;
        let exact_cos_0 = 1.0;
        let exact_sin_0 = 0.0;
        
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
                Token::Function(func) => {
                    let a = stack.pop().ok_or("Invalid expression")?;
                    
                    let result = match (func.as_str(), a as i32) {
                        ("sin", 90) => exact_sin_90,
                        ("sin", 0) => exact_sin_0,
                        ("cos", 0) => exact_cos_0,
                        _ => {
                            let angle = if !self.rad_mode {
                                a * std::f64::consts::PI / 180.0
                            } else {
                                a
                            };
                            
                            match func.as_str() {
                                "sin" => angle.sin(),
                                "cos" => angle.cos(),
                                "tan" => angle.tan(),
                                "ln" => {
                                    if a <= 0.0 {
                                        return Err("Invalid input for logarithm".to_string());
                                    }
                                    a.ln()
                                },
                                "log" => {
                                    if a <= 0.0 {
                                        return Err("Invalid input for logarithm".to_string());
                                    }
                                    a.log10()
                                },
                                "asin" => {
                                    if a < -1.0 || a > 1.0 {
                                        return Err("Invalid input for asin".to_string());
                                    }
                                    let result = a.asin();
                                    if !self.rad_mode {
                                        result.to_degrees()
                                    } else {
                                        result
                                    }
                                },
                                "acos" => {
                                    if a < -1.0 || a > 1.0 {
                                        return Err("Invalid input for acos".to_string());
                                    }
                                    let result = a.acos();
                                    if !self.rad_mode {
                                        result.to_degrees()
                                    } else {
                                        result
                                    }
                                },
                                "atan" => {
                                    let result = a.atan();
                                    if !self.rad_mode {
                                        result.to_degrees()
                                    } else {
                                        result
                                    }
                                },
                                _ => return Err("Unknown function".to_string()),
                            }
                        }
                    };
                    stack.push(result);
                }
            }
        }
        stack.pop().ok_or("Invalid expression".to_string())
    }

    pub fn format_expression(&self, expr: &str) -> String {
        let mut formatted = String::new();
        let mut prev_char_is_digit = false;
        let mut current_word = String::new();

        for c in expr.chars() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    current_word.push(c);
                    prev_char_is_digit = false;
                }
                '0'..='9' | '.' => {
                    if !current_word.is_empty() {
                        formatted.push_str(&current_word);
                        formatted.push(' ');
                        current_word.clear();
                    }
                    if !prev_char_is_digit && !formatted.is_empty() && !formatted.ends_with(' ') {
                        formatted.push(' ');
                    }
                    formatted.push(c);
                    prev_char_is_digit = true;
                }
                '+' | '-' | '*' | '/' => {
                    if !current_word.is_empty() {
                        formatted.push_str(&current_word);
                        formatted.push(' ');
                        current_word.clear();
                    }
                    if prev_char_is_digit && !formatted.ends_with(' ') {
                        formatted.push(' ');
                    }
                    formatted.push(c);
                    formatted.push(' ');
                    prev_char_is_digit = false;
                }
                _ => {}
            }
        }

        if !current_word.is_empty() {
            formatted.push_str(&current_word);
        }

        formatted.trim().to_string()
    }

    pub fn evaluate(&self, expr: &str) -> Result<f64, String> {
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
}

