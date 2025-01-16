// src/token.rs
#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Operator(char),
    Function(String),
}

