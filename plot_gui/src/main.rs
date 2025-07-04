pub mod rplt;

use rplt::figure::*;

fn main() {
    // This is your GUI entry point (e.g., egui/eframe window)
    let x: Vec<f64> = (0..100).map(f64::from).collect();
    let y: Vec<f64> = (0..100).map(f64::from).collect();

    let mut p = Figure::new(Layout { rows:2, columns:2});

    p.subplot(0,0).plot(&x, &y, Some(LineStyle::Line));
    p.subplot(0,0).plot(&x, &y, Some(LineStyle::Marker(MarkerStyle::Cross)));
    p.subplot(1,0).plot(&x, &y, Some(LineStyle::Marker(MarkerStyle::Dot)));

    let _ = p.show();
}

// use eframe::{egui, App, Frame};

// pub struct MyApp {
//     counter: i32,
// }

// impl Default for MyApp {
//     fn default() -> Self {
//         Self { counter: 0 }
//     }
// }

// impl App for MyApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("Hello from egui!");
//             if ui.button("Click me").clicked() {
//                 self.counter += 1;
//             }
//             ui.label(format!("Counter: {}", self.counter));
//         });
//     }
// }