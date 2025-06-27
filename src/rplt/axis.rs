use egui_plot::{Line, MarkerShape, Points};

use super::plotdata::*;

#[derive(Default)]
pub struct Axis {
    pub data: Vec<PlotData>,
    pub row: usize,
    pub column: usize,
}

impl Axis {
    fn new(row: usize, column: usize) -> Self {
        Self {
            data: Vec::new(),
            row: row,
            column: column,
        }
    }

    pub fn plot(&mut self, x: Vec<f64>, y: Vec<f64>, style: Option<LineStyle>) {
        self.data
            .push(PlotData::new(x, y, style.unwrap_or(LineStyle::Line)));
    }

    pub fn create_line(&self, points: Vec<[f64; 2]>) -> Line {
        Line::new("".to_string(), points)
            .name("Line")
            .style(egui_plot::LineStyle::Solid)
    }

    pub fn create_dot_line(&self, points: Vec<[f64; 2]>, marker_style: MarkerStyle) -> Points {
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
