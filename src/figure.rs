use super::Axis;
use eframe::egui::{self, Visuals};
use eframe::{App, Error, Frame, NativeOptions};
use egui::CentralPanel;
use egui_plot::Plot;

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

impl Default for LineStyle {
    fn default() -> Self {
        LineStyle::Line
    }
}

#[derive(Default)]
pub struct Layout {
    pub rows: usize,
    pub columns: usize,
}

pub struct Figure {
    axis: Vec<Axis<f64>>,
    layout: Layout,
    pub title : String
}

impl Figure {
    pub fn new(layout: Layout) -> Self {
        let mut axis = Vec::with_capacity(layout.rows * layout.columns);

        for row in 0..layout.rows {
            for col in 0..layout.columns {
                axis.push(Axis::new(row, col));
            }
        }

        Self {
            axis: axis,
            layout: layout,
            title: "".to_string(),
        }
    }

    pub fn show(self) -> Result<(), Error> {
        let options: NativeOptions = NativeOptions {
            centered: true,            
            ..NativeOptions::default()
        };

        // Wrap `self` in `Some` so we can `take()` it inside the closure
        let mut app = Some(self);

        eframe::run_native(
            "Line Plot Example",
            options,
            Box::new(move |_cc| Ok(Box::new(app.take().unwrap()))),
        )
    }

    pub fn subplot(&mut self, row: usize, col: usize) -> &mut Axis<f64> {
        self.axis
            .iter_mut()
            .find(|axis| axis.row == row && axis.column == col)
            .unwrap()
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
            title: "".to_string(),
        }
    }
}

impl App for Figure {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ctx.set_visuals(Visuals::dark());

        CentralPanel::default().show(ctx, |ui| {
            if !self.title.is_empty(){
                ui.heading(self.title.clone());
            }

            let available = ui.available_size();
            let plot_size = egui::Vec2::new(
                available.x / self.layout.columns as f32,
                available.y / self.layout.rows as f32,
            );

            for row in 0..self.layout.rows {
                ui.horizontal(|ui| {
                    
                    for col in 0..self.layout.columns {
                        let plot_id = format!("plot_{}_{}", row, col);

                        // Reserve space first
                        ui.allocate_ui_with_layout(
                            plot_size,
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                let current_axis = self.subplot(row, col);
                                
                                // set title
                                if !current_axis.title.is_empty() {
                                    ui.heading(current_axis.title.clone());
                                }

                                Plot::new(plot_id).show(ui, |plot_ui| {
                                    for pts in current_axis.data.iter() {
                                        let raw_points: Vec<[f64; 2]> = pts
                                            .x
                                            .iter()
                                            .cloned()
                                            .zip(pts.y.iter().cloned())
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
