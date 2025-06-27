use eframe::egui::{self, Visuals};
use eframe::{App, Error, Frame, NativeOptions};
use egui::CentralPanel;
use egui_plot::{Line, Plot};

use super::plotdata::LineStyle;

use super::axis::*;

#[derive(Default)]
pub struct Layout {
    pub rows: usize,
    pub columns: usize,
}

pub struct Figure {
    axis: Vec<Axis>,
    layout: Layout,
}

impl Figure {
    pub fn new(layout: Layout) -> Self {
        let mut axis = Vec::with_capacity(layout.rows * layout.columns);

        for row in 0..layout.rows {
            for col in 0..layout.columns {
                axis.push(Axis {
                    row,
                    column: col,
                    data: Vec::new(),
                });
            }
        }

        Self {
            axis: axis,
            layout: layout,
        }
    }

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

    pub fn subplot(&mut self, row: usize, col: usize) -> Option<&mut Axis> {
        self.axis
            .iter_mut()
            .find(|axis| axis.row == row && axis.column == col)
    }
}

impl Default for Figure {
    fn default() -> Self {
        Self {
            axis: Vec::new(),
            layout: Layout {
                rows: 1,
                columns: 1,
            },
        }
    }
}

impl App for Figure {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ctx.set_visuals(Visuals::dark());

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("2x2 Plot Grid");

            let available = ui.available_size();
            let plot_size = egui::Vec2::new(available.x / 2.0, available.y / 2.0);

            for row in 0..self.layout.rows {
                ui.horizontal(|ui| {
                    for col in 0..self.layout.columns {
                        let plot_id = format!("plot_{}_{}", row, col);

                        // Reserve space first
                        ui.allocate_ui_with_layout(
                            plot_size,
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                ui.heading("text");

                                Plot::new(plot_id).show(ui, |plot_ui| {
                                    let current_axis = self.subplot(row, col).unwrap();
                                    for pts in current_axis.data.iter() {
                                        let raw_points: Vec<[f64; 2]> = pts
                                            .x
                                            .clone()
                                            .into_iter()
                                            .zip(pts.y.clone().into_iter())
                                            .map(|(x, y)| [x, y])
                                            .collect();

                                        match pts.style {
                                            LineStyle::Line => {
                                                let line =
                                                    current_axis.create_line(raw_points.clone());
                                                plot_ui.line(line);
                                            }
                                            LineStyle::Marker(marker_style) => {
                                                let dots = current_axis.create_dot_line(
                                                    raw_points.clone(),
                                                    marker_style,
                                                );
                                                plot_ui.points(dots);
                                            }
                                        }
                                    }
                                });
                            },
                        );
                    }
                });
            }
        });
    }
}
