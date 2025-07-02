use egui_plot::{Line, MarkerShape, Points};

use super::plotdata::*;
use super::figure::*;

#[derive(Default)]
pub struct Axis {
    pub data: Vec<PlotData>,
    pub row: usize,
    pub column: usize,
}

impl Axis {
    pub fn new(row: usize, column: usize) -> Self {
        Self {
            data: Vec::new(),
            row: row,
            column: column,
        }
    }

    pub fn plot(&mut self, x: &Vec<f64>, y: &Vec<f64>, style: Option<LineStyle>) {
        self.data
            .push(PlotData::new(x.clone(), y.clone(), style.unwrap_or(LineStyle::Line)));
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

#[derive(Clone)]
pub struct PlotData {    
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub style: LineStyle,
}

impl PlotData {
    pub fn new() -> Self {
        PlotData {
            x: Vec::new(),
            y: Vec::new(),
            style: LineStyle::Line,            
        }
    }

    pub fn append(&mut self, x: f64, y: f64) {
        self.x.push(x.clone());
        self.y.push(y.clone());
        self.style = LineStyle::Marker(MarkerStyle::Cross);
    }
}

impl Default for PlotData {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), style: Default::default() }
    }
}