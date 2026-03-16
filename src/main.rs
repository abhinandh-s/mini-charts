#![allow(unused)]

use std::fmt::Display;

use mini_charts::{Line, Mocha, Polygon, RenderSvg as _, Svg, Text};
use num_traits::{Float, FromPrimitive, ToPrimitive};

/// Custom for svg, not for anything else
fn polar_to_cartesian(r: f64, deg: f64, scale: f64) -> (f64, f64) {
    let rad = (deg - 90.0).to_radians();
    let x = r * rad.cos() * scale;
    let y = r * rad.sin() * scale;
    (x, y)
}

struct Point {
    x: f64,
    y: f64,
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

struct RadarChart {
    size: f64,
    center: Point,
    radius: f64,
    data: Vec<Series>,
    svg: Svg,
    theme: RadarTheme,
    axes_labels: Vec<String>,
}

struct RadarTheme {
    bg: Option<&'static str>,
    label: Option<&'static str>,
    legend: Option<&'static str>,
    axis: Option<&'static str>,
    scale: Option<&'static str>,
}

impl Default for RadarTheme {
    fn default() -> Self {
        Self {
            bg: Some(Mocha::CRUST),
            label: Some(Mocha::TEXT),
            legend: Some(Mocha::TEXT),
            axis: Some(Mocha::SUBTEXT0),
            scale: Some(Mocha::SUBTEXT0),
        }
    }
}

impl RadarChart {
    fn new(size: usize, padding: usize) -> Self {
        let s = size as f64;
        let center = s / 2.0;
        let radius = center - padding as f64;
        Self {
            size: s,
            center: Point {
                x: center,
                y: center,
            },
            radius,
            data: vec![],
            svg: Svg::new(s, s),
            theme: RadarTheme::default(),
            axes_labels: vec![],
        }
    }

    fn to_cordinates(&self, deg: f64, scale: f64) -> Point {
        let (x, y) = polar_to_cartesian(self.radius, deg, scale);
        Point { x, y }
    }

    #[deprecated]
    fn degree_in_between(&self, data: &[f64]) -> f64 {
        360.0 / data.len() as f64
    }

    /// Slices the figure into `n` equal parts, and returns the angle between them.
    ///
    /// where,
    ///     n = number of sides of the figure
    fn deg_in_between<F: Into<f64>>(&self, n: F) -> f64 {
        360.0 / n.into()
    }

    #[deprecated]
    fn no_of_sides(&self) -> usize {
        self.data.len()
    }

    // pub fn edge_point(&self, data: usize, total_axes: usize) -> Point {
    //     let deg = i as f64 * self.degree_in_between(data);
    //     self.to_cordinates(deg, 1.0)
    // }

    fn data_to_polygon_points(&self, data: &[f64]) -> String {
        let n = self.no_of_edges();
        data.iter()
            .enumerate()
            .map(|(i, &val)| {
                let deg = i as f64 * self.deg_in_between(n as f64);
                self.to_cordinates(deg, val).to_string()
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn no_of_edges(&self) -> usize {
        self.data
            .first()
            .map(|s| s.values.len())
            .unwrap_or_default()
    }

    fn draw_axis(&mut self) {
        let n = self.no_of_edges();
        (0..n).for_each(|(i)| {
            let deg = i as f64 * self.deg_in_between(n as f64);
            let edge = self.to_cordinates(deg, 1.0);
            self.draw_line_from_center(edge);
        });
    }

fn draw_labels(&mut self) {
    let n = self.no_of_edges();
    let n_f64 = n as f64;

    (0..n).for_each(|i| {
        if let Some(name) = self.axes_labels.get(i) {
            let deg = i as f64 * self.deg_in_between(n_f64);
            
            // 1. Calculate the coordinates slightly outside the chart radius (e.g., 110.0)
            let edge = self.to_cordinates(deg, 1.1);

            // 2. Convert degrees to radians for alignment math
            // SVG 0 degrees is usually the 3 o'clock position
            let rad = deg.to_radians();
            let cos_val = rad.cos();
            let sin_val = rad.sin();

            // 3. Horizontal Alignment (text-anchor)
            let anchor = if cos_val.abs() < 0.1 {
                "middle" // Top or Bottom
            } else if cos_val > 0.0 {
                "start"  // Right side
            } else {
                "end"    // Left side
            } ;

            // 4. Vertical Alignment (dominant-baseline)
            let baseline = if sin_val.abs() < 0.1 {
                "middle"      // Left or Right
            } else if sin_val > 0.0 {
                "hanging"     // Bottom (text hangs below point)
            } else {
                "alphabetic"  // Top (text sits above point)
            };

            Text {
                x: edge.x,
                y: edge.y,
                content: name.clone(),
                style: format!(
                    "fill:{};font-size:12px;text-anchor:{};dominant-baseline:{};font-family:sans-serif;", 
                    Mocha::TEXT, anchor, baseline
                ),
            }.render(&mut self.svg);
        }
    });
}
    fn draw_svg_comments(&mut self, comment: &str) {
        self.svg
            .push_raw(format!("\n<!-- {} -->\n", comment).as_str());
    }

    /// Draws a line from center of the circle to given point.
    fn draw_line_from_center(&mut self, edge: Point) {
        // Axis Line
        Line {
            x1: 0.0,
            y1: 0.0,
            x2: edge.x,
            y2: edge.y,
            style: format!(
                "stroke:{};stroke-opacity:0.3;stroke-dasharray:2;",
                self.theme.axis.unwrap_or_default()
            ),
        }
        .render(&mut self.svg)
    }

    fn set_data(&mut self, data: Vec<Series>) {
        self.data = data;
    }

    pub fn add_series(&mut self, value: Series) {
        self.data.push(value)
    }

    fn draw_legend(&mut self) {
        let padding = 10.0;
        let half_size = -self.size / 2.0;
        let x_start = half_size + padding;
        let mut y_start = half_size + padding;
        let spacing = 18.0;

        for series in &self.data {
            // Colored indicator (Square)
            self.svg.push_raw(&format!(
                r#"<rect x="{}" y="{}" width="08" height="08" rx="2" fill="{}" />"#,
                x_start, y_start, series.color
            ));

            // Series Label
            Text {
                x: x_start + 15.0,
                y: y_start + 06.7,
                content: series.name.clone(),
                style: format!(
                    "fill:{};font-size:08px;font-family:sans-serif;font-weight:bold;",
                    Mocha::TEXT
                ),
            }
            .render(&mut self.svg);

            y_start += spacing;
        }
    }

    fn render(&mut self) -> String {
        let c = Polygon {
            points: "10,20 13,2 70,22".into(),
            style: "fill:red;".into(),
        };
        self.draw_bg();
        self.draw_svg_comments("draw axis");
        self.draw_axis();
        self.draw_svg_comments("draw scale");
        self.draw_scale(4);
        self.draw_svg_comments("draw labels");
        self.draw_labels();
        self.draw_svg_comments("draw legends");
        self.draw_legend();

        for series in &self.data {
            Polygon {
                points: self.data_to_polygon_points(&series.values),
                style: format!(
                    "fill:{};fill-opacity:0.3;stroke:{};stroke-width:1;",
                    series.color, series.color
                ),
            }.render(&mut self.svg);
        }

        self.svg.finish()
    }

    fn draw_bg(&mut self) {
        let half_size = self.size / 2.0;
    let min_x = -half_size;
    let min_y = -half_size;

        self.svg.push_raw(&format!(
            "  <rect id=\"chart-bg\" x=\"{min_x}\" y=\"{min_y}\" width=\"100%\" height=\"100%\" fill=\"{}\" />\n",
            self.theme.bg.unwrap_or_default()
        ));
    }

    fn draw_scale(&mut self, n: usize) {
        let n_edges = self.no_of_edges();
        // 2. Grid Polygons (Concentric Webs)
        for level in 1..=n {
            let scale = level as f64 / n as f64;
            let scale_vec = vec![scale; n_edges];
            Polygon {
                points: self.data_to_polygon_points(&scale_vec),
                style: format!(
                    "fill:none;stroke:{};stroke-opacity:0.2;",
                    self.theme.scale.unwrap_or_default()
                ),
            }
            .render(&mut self.svg);
        }
    }

    fn set_axes_labels(&mut self, axes_labels: Vec<String>) {
        self.axes_labels = axes_labels;
    }
}

fn main() {
    let portfolio_a = [0.8, 0.6, 0.9, 0.5, 0.7, 0.4];
    let portfolio_b = [0.6, 0.7, 0.5, 0.8, 0.6, 0.5];
    let portfolio_c = [0.1, 0.3, 0.7, 0.2, 0.9, 0.8];

    let mut radar = RadarChart::new(300, 40);
    radar.add_series(Series {
        name: "Alpha".into(),
        values: portfolio_a.to_vec(),
        color: Mocha::RED.to_owned(),
    });

        radar.add_series(Series {
        name: "Beta".into(),
        values: portfolio_b.to_vec(),
        color: Mocha::BLUE.to_owned(),
    });
    radar.add_series(Series {
        name: "Theta".into(),
        values: portfolio_c.to_vec(),
        color: Mocha::GREEN.to_owned(),
    });

    let labels = vec![
        "Speed".into(),
        "Power".into(),
        "Agility".into(),
        "Stamina".into(),
        "Range".into(),
        "Will".into(),
    ];
    radar.set_axes_labels(labels);

    let svg_output = radar.render();
    std::fs::write("radar.svg", svg_output).expect("Unable to write file");
    println!("wrote SVG to radar.svg.");
}
