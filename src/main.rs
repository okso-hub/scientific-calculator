use eframe::egui;
use simple_gui::CalculatorApp;  // assuming your crate is named simple_gui

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