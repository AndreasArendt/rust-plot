use eframe::egui::{self, CentralPanel, Key, Visuals};
use egui_plotter::{Chart, MouseConfig};
use plotters::prelude::*;
use std::ops::Range;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "ParaChart Example",
        native_options,
        Box::new(|cc| Ok(Box::new(ParaChart::new(cc)))),
    )
    .unwrap();
}

struct ParaChart {
    chart: Chart<(Range<f32>, Range<f32>)>,
    zoom_factor: f32,
}

impl ParaChart {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Disable feathering as it causes artifacts
        let context = &cc.egui_ctx;

        let zoom_factor = 1.0;

        context.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });

        // Also enable light mode
        context.set_visuals(Visuals::light());

        // We use data to adjust the range of the chart. This can be useful for
        // line plots where the X represents time and we want to play through
        // the X, but that is not what we are using it for here
        let chart = Chart::new((-3f32..3f32, -0.5f32..3f32))
            //.mouse(MouseConfig::enabled())
            .mouse(MouseConfig::enabled().zoom(false).drag(false))
            .builder_cb(Box::new(|area, _t, ranges| {
                // Build a chart like you would in any other plotter chart.
                // The drawing area and ranges are provided by the callback,
                // but otherwise everything else is the same.

                let (x_range, y_range) = ranges;

                let mut chart = ChartBuilder::on(area)
                    .caption("y=x^2", ("sans-serif", 50).into_font())
                    .margin(5)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(x_range.to_owned(), y_range.to_owned())
                    .unwrap();

                chart.configure_mesh().draw().unwrap();

                let step = 0.01;
                let x_start = -50.0 * x_range.end;
                let x_end = 50.0 * x_range.end;
                let x_values = (0..=((x_end - x_start) / step) as usize).map(|i| {
                    x_start + i as f32 * step                    
                });

                let line = LineSeries::new(x_values.map(|x| (x, x * x)), BLACK);

                chart
                    .draw_series(line)
                    .unwrap()
                    .label("y = x^2")
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

                chart
                    .configure_series_labels()
                    .background_style(WHITE.mix(0.8))
                    .border_style(BLACK)
                    .draw()
                    .unwrap();
            }));

        Self { chart, zoom_factor }
    }
}

impl eframe::App for ParaChart {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // Press 1 for the range -1..1, 2 for -2..2, 3 for -3..3
            ui.input(|input| {
                if input.raw_scroll_delta.y > 0.0 {
                    self.zoom_factor *= 0.9;
                } else if input.raw_scroll_delta.y < 0.0 {
                    self.zoom_factor *= 1.1;
                }

                let min_x = -3f32 * self.zoom_factor;
                let max_x = 3f32 * self.zoom_factor;

                let min_y = -0.5f32 * self.zoom_factor;
                let max_y = 3f32 * self.zoom_factor;

                *self.chart.get_data_mut() = (min_x..max_x, min_y..max_y);
            });

            self.chart.draw(ui);
        });
    }
}
