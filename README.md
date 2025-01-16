# Scientific Calculator
![rust](https://github.com/okso-hub/scientific-calculator/actions/workflows/rust.yml/badge.svg)

A scientific calculator built with Rust and egui framework, offering a modern graphical interface for mathematical calculations.

## Features (and whether they work)

- [x] Basic arithmetic operations with two arguments (+, -, *, /)
- [ ] Scientific functions (sin, cos, tan, log)
- [x] Trigonometric functions in both RAD/DEG modes
- [x] Constants (π, e)
- [x] Error handling and input validation
- [x] Clean and intuitive GUI

## Technologies

- Rust programming language
- eframe/egui for GUI
- num crate for numerical computations
- libm for mathematical functions

## Installation

1. Ensure you have Rust installed. If not, install from [rustup.rs](https://rustup.rs/)
2. Clone the repository:
```bash
git clone https://github.com/yourusername/scientific_calculator.git
cd scientific_calculator
```
3. Build the project:
```bash
cargo build --release
```

## Usage
Run the calculator:
```bash
cargo run --release
```

## Basic Operations
- Use number buttons (0-9) for input
- Click operators (+, -, *, /) for calculations
- Press '=' to evaluate
- 'C' clears the display
- '±' changes number sign

## Scientific Functions
- Trigonometric: sin, cos, tan, asin, acos, atan
- Logarithmic: ln, log
- Constants: π (pi), e (euler's number)
- Switch between RAD/DEG modes for trigonometry

## Testing
Run the test suite:
```bash
cargo test
```

Tests cover:
- Calculator initialization
- Basic arithmetic operations
- Expression formatting
- Error handling

## Preview
<img width="512" alt="image" src="https://github.com/user-attachments/assets/8c507cad-d9d2-4701-b874-ab9180ebf4d5" />
