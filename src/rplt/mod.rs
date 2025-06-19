use std::os::raw;

use eframe::egui::{self, Visuals};
use eframe::{App, Error, Frame, NativeOptions};
use egui::CentralPanel;
use egui_plot::{Line, Plot, PlotPoints, Points};

#[derive(Clone)]
pub struct PlotData {
    x_data: Vec<f64>,
    y_data: Vec<f64>,
}

impl PlotData {
    pub fn new(x: Vec<f64>, y: Vec<f64>) -> Self {
        PlotData {
            x_data: x,
            y_data: y,
        }
    }
}

pub struct Rplot {
    data: Vec<PlotData>,
}

impl Rplot {
    pub fn show(self) -> Result<(), Error> {
        let options = NativeOptions::default();

        // Wrap `self` in `Some` so we can `take()` it inside the closure
        let mut app = Some(self);

        eframe::run_native(
            "Line Plot Example",
            options,
            Box::new(move |_cc| {
                // Move ownership into the App
                Ok(Box::new(app.take().unwrap()))
            }),
        )
    }

    pub fn plot(&mut self, x: Vec<f64>, y: Vec<f64>) {
        let plt_data = PlotData::new(x, y);
        self.data.push(plt_data);
    }
}

impl Default for Rplot {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

impl App for Rplot {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ctx.set_visuals(Visuals::light());

        CentralPanel::default().show(ctx, |ui| {
            Plot::new("line_plot").show(ui, |plot_ui| {
                for pts in self.data.iter() {
                    let raw_points: Vec<[f64; 2]> = pts
                        .x_data
                        .clone()
                        .into_iter()
                        .zip(pts.y_data.clone().into_iter())
                        .map(|(x, y)| [x, y])
                        .collect();

                    let line = Line::new("".to_string(), raw_points.clone())
                        .name("Line")
                        .style(egui_plot::LineStyle::Solid);

                    let dots = Points::new("".to_string(), raw_points.clone())
                        .radius(2.0)
                        .name(format!("Dots"));

                    plot_ui.line(line);
                    plot_ui.points(dots);
                }
            });
        });
    }
}
