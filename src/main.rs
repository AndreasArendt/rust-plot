use rplt::{create_figure, figure::{Layout, LineStyle, MarkerStyle}};

fn main() {
    let x: Vec<f64> = (0..100).map(f64::from).collect();
    let y: Vec<f64> = (0..100).map(f64::from).collect();

    let mut p = create_figure(Layout{rows:3, columns:1});
    p.title = "Figure Title".to_string();

    p.subplot(0,0).plot(&x, &y, Some(LineStyle::Line));
    p.subplot(0,0).plot(&x, &y, Some(LineStyle::Marker(MarkerStyle::Cross)));
    p.subplot(1,0).plot(&x, &y, Some(LineStyle::Marker(MarkerStyle::Dot)));

    p.subplot(0,0).title = "Title 0,0".to_string();
    p.subplot(1,0).title = "Title 2,0".to_string();
    p.subplot(2,0).title = "Title 3,0".to_string();


    let _ = p.show();
}
