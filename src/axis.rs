use egui_plot::{Line, MarkerShape, Points};

use super::figure::*;

#[derive(Default)]
pub struct Axis<T: Clone + Default> {
    pub data: Vec<PlotData<T>>,
    pub row: usize,
    pub column: usize,
    pub title: String,
}

impl Axis<f64> {
    pub fn plot<T>(&mut self, x: &[T], y: &[T], style: Option<LineStyle>)
    where
        T: Into<f64> + Copy,
    {
        let x_f64: Vec<f64> = x.iter().map(|&v| v.into()).collect();
        let y_f64: Vec<f64> = y.iter().map(|&v| v.into()).collect();
        self.data.push(PlotData::new(
            x_f64,
            y_f64,
            style.unwrap_or(LineStyle::Line),
        ));
    }
}

impl<T: Clone + Default> Axis<T> {
    pub fn new(row: usize, column: usize) -> Self {
        Self {
            data: Vec::new(),
            row,
            column,
            title: "".to_string(),
        }
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
pub struct PlotData<T> {
    pub x: Vec<T>,
    pub y: Vec<T>,
    pub style: LineStyle,
}

impl<T> PlotData<T> {
    pub fn new(x: Vec<T>, y: Vec<T>, style: LineStyle) -> Self {
        PlotData {
            x,
            y,
            style,
        }
    }
}

impl<T> Default for PlotData<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            style: Default::default(),
        }
    }
}
