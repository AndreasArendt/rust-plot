pub mod figure;
pub mod axis;
use crate::figure::*;
use crate::axis::*;

pub fn create_figure(layout: Layout) -> Figure {
    Figure::new(layout)
}