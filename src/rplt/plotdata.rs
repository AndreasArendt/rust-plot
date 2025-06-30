use super::figure::*;

#[derive(Clone)]
pub struct PlotData {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub style: LineStyle,
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
