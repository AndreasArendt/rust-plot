pub mod rplt;

use rplt::figure::*;

pub fn create_figure(layout: Layout) -> Figure {
    Figure::new(layout)
}