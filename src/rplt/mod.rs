use eframe::egui::{self, Visuals};
use eframe::{App, Error, Frame, NativeOptions};
use egui::CentralPanel;
use egui_plot::{Line, Plot, PlotPoints};

pub struct Rplot;

impl Rplot {
    pub fn show(self) -> Result<(), Error> {        
        let mut options = NativeOptions::default();
        options.centered = true;
        
        eframe::run_native(
            "Line Plot Example",
            options,
            Box::new(|_cc| Ok(Box::new(Rplot::default()))),
        )
    }
}

impl App for Rplot {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // set lightmode
        ctx.set_visuals(Visuals::light());

        CentralPanel::default().show(ctx, |ui| {
            let data: Vec<f64> = (0..100).map(|x| (x as f64 / 10.0).sin()).collect();
            let points: PlotPoints = data
                .iter()
                .enumerate()
                .map(|(i, &y)| [i as f64, y])
                .collect();

            let line = Line::new("".to_string(), points);
            Plot::new("line_plot").show(ui, |plot_ui| {
                plot_ui.line(line);
            });
        });
    }
}


impl Default for Rplot {
    fn default() -> Self {
        Self
    }
}