pub mod charts;
mod colors;
mod svg;
mod themes;

pub use charts::*;
pub use colors::*;
use std::fmt::Display;
pub use svg::*;

/// Custom for svg.
///
/// where,
///     center = (0,0)
///
/// r, deg => will give point on circle.
/// if scale == 1.0 => the coordinate is on circle.
/// scale => helps to move the coordinate to and fro in same direction.
pub(crate) fn polar_to_cartesian(r: f64, deg: f64, scale: f64) -> (f64, f64) {
    let rad = (deg - 90.0).to_radians();
    let x = r * rad.cos() * scale;
    let y = r * rad.sin() * scale;
    (x, y)
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    fn from_polar(r: f64, deg: f64, scale: f64) -> Self {
        let (x, y) = polar_to_cartesian(r, deg, scale);
        Point { x, y }
    }
}

// Gives `.to_string()` for free
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2},{:.2}", self.x, self.y)
    }
}

pub struct Series {
    pub name: String,
    pub values: Vec<f64>,
    pub color: String,
}
