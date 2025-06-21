use eframe::egui::{self, Visuals};
use eframe::{App, Error, Frame, NativeOptions};
use egui::CentralPanel;
use egui_plot::{Line, MarkerShape, Plot, Points};

#[derive(Clone, Copy)]
pub enum MarkerStyle {
    Dot,
    Cross,
}

#[derive(Clone)]
pub enum LineStyle {
    Line,
    Marker(MarkerStyle),
}

#[derive(Clone)]
pub struct PlotData {
    x: Vec<f64>,
    y: Vec<f64>,
    style: LineStyle,
}

impl PlotData {
    pub fn new(x: Vec<f64>, y: Vec<f64>, style: LineStyle) -> Self {
        PlotData {
            x: x,
            y: y,
            style: style,
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
            Box::new(move |_cc| Ok(Box::new(app.take().unwrap()))),
        )
    }

    pub fn plot(&mut self, x: Vec<f64>, y: Vec<f64>, style: Option<LineStyle>) {
        self.data
            .push(PlotData::new(x, y, style.unwrap_or(LineStyle::Line)));
    }

    fn create_line(&self, points: Vec<[f64; 2]>) -> Line {
        Line::new("".to_string(), points)
            .name("Line")
            .style(egui_plot::LineStyle::Solid)
    }

    fn create_dot_line(&self, points: Vec<[f64; 2]>, marker_style: MarkerStyle) -> Points {
        let style = match marker_style {
            MarkerStyle::Dot => MarkerShape::Circle,
            MarkerStyle::Cross => MarkerShape::Cross,
        };

        Points::new("".to_string(), points)
            .radius(4.0)
            .name(format!("Dots"))
            .shape(style)
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
                        .x
                        .clone()
                        .into_iter()
                        .zip(pts.y.clone().into_iter())
                        .map(|(x, y)| [x, y])
                        .collect();

                    match pts.style {
                        LineStyle::Line => {
                            let line = self.create_line(raw_points.clone());
                            plot_ui.line(line);
                        }
                        LineStyle::Marker(marker_style) => {
                            let dots = self.create_dot_line(raw_points.clone(), marker_style);
                            plot_ui.points(dots);
                        }
                    }
                }
            });
        });
    }
}
