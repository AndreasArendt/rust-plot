pub mod rplt;

use rplt::figure::*;
use rplt::plotdata::*;

fn main() {
    let x: Vec<f64> = (0..100).map(f64::from).collect();
    let y: Vec<f64> = (0..100).map(f64::from).collect();

    let mut p = Figure::new(Layout { rows:2, columns:2});

    p.subplot(0,0).unwrap().plot(x.clone(), y.clone(), Some(LineStyle::Line));
    p.subplot(0,0).unwrap().plot(x.clone(), y.clone(), Some(LineStyle::Marker(MarkerStyle::Cross)));
    p.subplot(1,0).unwrap().plot(x.clone(), y.clone(), Some(LineStyle::Marker(MarkerStyle::Dot)));

    let _ = p.show();
}
