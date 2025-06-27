# rplt

rplt is a lightweight plotting library for Rust. It is designed to be minimal, fast to get started with, and easy to integrate into small or embedded Rust projects where simplicity matters.

## ✨ Features
* Minimal dependencies
* Simple API for creating subplots
* Supports line and marker plots
* Clean and readable syntax
* Works out of the box — no setup required

## 🚀 Quick Start
Minimal working example showing a 2x2 subplot layout to plot some data:

```
pub mod rplt;

use rplt::figure::*;
use rplt::plotdata::*;

fn main() {
    let x: Vec<f64> = (0..100).map(f64::from).collect();
    let y: Vec<f64> = (0..100).map(f64::from).collect();

    let mut p = Figure::new(Layout { rows: 2, columns: 2 });

    p.subplot(0, 0).unwrap().plot(x.clone(), y.clone(), Some(LineStyle::Line));
    p.subplot(0, 0).unwrap().plot(x.clone(), y.clone(), Some(LineStyle::Marker(MarkerStyle::Cross)));
    p.subplot(1, 0).unwrap().plot(x.clone(), y.clone(), Some(LineStyle::Marker(MarkerStyle::Dot)));

    let _ = p.show();
}
```

## 📦 Installation
Add rplt to your Cargo.toml (to be released in after first beta to crates.io):
```
[dependencies]
rplt = { git = "https://github.com/AndreasArendt/rust-plot" }
```

## 📚 API Overview

    Figure::new(Layout) — Create a figure with subplots

    subplot(row, col) — Access specific subplot

    plot(x, y, style) — Plot data with optional style

    show() — Render the figure

Available Line Styles (to be extended)

    LineStyle::Line

    LineStyle::Marker(MarkerStyle::Dot)

    LineStyle::Marker(MarkerStyle::Cross)

## 🛠️ Roadmap
* Add support for legends, labels, and titles
* Export to image formats
* Interactive plots (custom linking, limiting, etc.)
* Custom color and style themes

## 🤝 Contributing

Contributions, bug reports, and feature suggestions are welcome! Please open an issue or submit a pull request.

## 📄 License

MIT License © [Andreas Arendt]