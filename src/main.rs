pub mod rplt;

use rplt::figure::*;

fn main() {
    let x: Vec<f64> = (0..100).map(f64::from).collect();
    let y: Vec<f64> = (0..100).map(f64::from).collect();

    let mut p = Figure::new(Layout { rows:2, columns:2});

    p.subplot(0,0).plot(&x, &y, Some(LineStyle::Line));
    p.subplot(0,0).plot(&x, &y, Some(LineStyle::Marker(MarkerStyle::Cross)));
    p.subplot(1,0).plot(&x, &y, Some(LineStyle::Marker(MarkerStyle::Dot)));

    let _ = p.show();
}
