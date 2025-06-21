use crate::rplt::{LineStyle};

pub mod rplt;

fn main() {

    let x: Vec<f64> = (0..100).map(f64::from).collect();
    let y: Vec<f64> = (0..100).map(f64::from).collect();

    let mut p = rplt::Rplot::default();
    p.plot(x,y, Some(LineStyle::Marker(rplt::MarkerStyle::Cross)));
    let _ = p.show();
}
