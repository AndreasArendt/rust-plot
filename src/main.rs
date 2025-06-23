pub mod rplt;

use rplt::figure::Figure;
use rplt::plotdata::{LineStyle, MarkerStyle};

fn main() {
    let x: Vec<f64> = (0..100).map(f64::from).collect();
    let y: Vec<f64> = (0..100).map(f64::from).collect();

    let mut p = Figure::default();
    p.plot(
        x.clone(),
        y.clone(),
        Some(LineStyle::Marker(MarkerStyle::Cross)),
    );
    p.plot(x.clone(), y.clone(), Some(LineStyle::Line));
    let _ = p.show();
}
