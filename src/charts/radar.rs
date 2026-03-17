use crate::*;

/// # Invariance
///
/// 1. center = (0,0)
///
/// This do all the maths
#[derive(Default)]
pub struct RadarLayout {
    // for scaling the chart
    //
    // width = size
    // height = size
    pub size: f64,
    // Invariance: center = (0,0)
    //
    // radius is != size / 2, cuz we takes padding,
    // hence a seperate field
    pub radius: f64,
    // let portfolio_a = &[0.8, 0.6, 0.9, 0.5, 0.7, 0.4];
    //
    // series_len = 6 - `(portfolio_a.len()`
    pub series_len: usize,
}

impl RadarLayout {
    pub fn get_point(&self, deg: f64, scale: f64) -> Point {
        Point::from_polar(self.radius, deg, scale)
    }

    /// Slices the figure into `n` equal parts, and returns the angle between them.
    ///
    /// where,
    ///     n = number of sides of the figure
    pub fn deg_in_between(&self) -> f64 {
        360.0 / self.series_len as f64
    }

    pub fn axis_count(&self) -> usize {
        self.series_len
    }

    // scale_values = 0.0..1.0
    pub fn data_to_points(&self, scale_values: &[f64]) -> String {
        scale_values
            .iter()
            .enumerate()
            .map(|(i, &scale)| {
                let deg = i as f64 * self.deg_in_between();
                self.get_point(deg, scale).to_string()
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// this do all the rendering
pub struct RadarConfig {
    // scale_n: usize
}
pub struct RadarChart {
    pub data: Vec<Series>,
    pub svg: Svg,
    pub theme: RadarTheme,
    pub axes_labels: Vec<String>,
    pub layout: RadarLayout,
}

pub struct RadarTheme {
    pub bg: Option<&'static str>,
    pub label: Option<&'static str>,
    pub legend: Option<&'static str>,
    pub axis: Option<&'static str>,
    pub scale: Option<&'static str>,
    pub anim: bool,
    pub anim_duration: usize,
    pub stroke_width: usize,
}

impl Default for RadarTheme {
    fn default() -> Self {
        Self {
            bg: Some(Mocha::CRUST),
            label: Some(Mocha::TEXT),
            legend: Some(Mocha::TEXT),
            axis: Some(Mocha::SUBTEXT0),
            scale: Some(Mocha::SUBTEXT0),
            anim: false,
            anim_duration: 0,
            stroke_width: 1,
        }
    }
}

impl RadarChart {}

impl RadarChart {
    pub fn new(size: usize) -> Self {
        let padding = size / 10;
        let s = size as f64;
        let center = s / 2.0;
        let radius = center - padding as f64;
        let theme = RadarTheme {
            stroke_width: size / 250,
            ..Default::default()
        };
        Self {
            data: vec![],
            svg: Svg::new(-s / 2.0, -s / 2.0, s, s),
            theme,
            axes_labels: vec![],
            // TODO: hardcoded series_len
            layout: RadarLayout {
                size: s,
                radius,
                series_len: 0,
            },
        }
    }

    fn draw_axis(&mut self) {
        let n = self.layout.axis_count();
        (0..n).for_each(|i| {
            let deg = i as f64 * self.layout.deg_in_between();
            let edge = self.layout.get_point(deg, 1.0);
            self.draw_line_from_center(edge);
        });
    }

    fn draw_labels(&mut self) {
        let n = self.layout.axis_count();
        let font_size = self.layout.size / 39.0;

        (0..n).for_each(|i| {
        if let Some(name) = self.axes_labels.get(i) {
            let deg = i as f64 * self.layout.deg_in_between();
            // 1. Position slightly outside the chart (1.1x radius)
            let edge = self.layout.get_point(deg, 1.03);

            // 2. We use the adjusted angle to match polar_to_cartesian logic
            // Because you offset by -90 in polar_to_cartesian, 
            // we do the same here to find the visual direction.
            let rad = (deg - 90.0).to_radians();
            let cos_val = rad.cos();
            let sin_val = rad.sin();

            // 3. Horizontal Alignment (text-anchor)
            // If the point is significantly left or right, align accordingly.
            let anchor = if cos_val.abs() < 0.1 {
                "middle" 
            } else if cos_val > 0.0 {
                "start"  
            } else {
                "end"    
            };

            // 4. Vertical Alignment (dominant-baseline)
            let baseline = if sin_val.abs() < 0.1 {
                "middle"
            } else if sin_val > 0.0 {
                "hanging"    // Bottom: text hangs below point
            } else {
                "alphabetic" // Top: text sits above point
            };

            Text {
                x: edge.x,
                y: edge.y,
                content: name.clone(),
                style: format!(
                    "fill:{};font-size:{}px;text-anchor:{};dominant-baseline:{};font-family:ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,liberation mono,monospace;", 
                    Mocha::TEXT, font_size , anchor, baseline
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
                "stroke:{0};stroke-opacity:0.3;stroke-dasharray:{1};stroke-width:{1};",
                self.theme.axis.unwrap_or_default(),
                self.theme.stroke_width
            ),
        }
        .render(&mut self.svg)
    }

    pub fn add_series(&mut self, series: Series) {
        if self.layout.series_len == 0 {
            self.layout.series_len = series.values.len();
        }
        self.data.push(series)
    }

    fn draw_legend(&mut self) {
        let padding = 10.0;
        let half_size = -self.layout.size / 2.0;
        let x_start = half_size + padding;
        let mut y_start = half_size + padding;
        let spacing = self.layout.size / 20.0;

        let font_size = self.layout.size / 36.0;

        for series in &self.data {
            // Colored indicator (Square)
            self.svg.push_raw(&format!(
                "<rect x=\"{0}\" y=\"{1}\" width=\"{fs}\" height=\"{fs}\" rx=\"2\" fill=\"{2}\" />",
                x_start,
                y_start,
                series.color,
                fs = font_size
            ));

            // Series Label
            Text {
                x: x_start + spacing,
                y: y_start + (spacing / 2.0),
                content: series.name.clone(),
                style: format!(
                    "fill:{};font-size:{}px;font-family:sans-serif;font-weight:bold;",
                    Mocha::TEXT,
                    font_size
                ),
            }
            .render(&mut self.svg);

            y_start += spacing;
        }
    }

    pub fn render(&mut self) -> String {
        self.draw_bg();
        self.draw_svg_comments("draw axis");
        self.draw_axis();
        self.draw_svg_comments("draw scale");
        self.draw_grid_levels(4);
        self.draw_svg_comments("draw labels");
        self.draw_labels();
        self.draw_svg_comments("draw legends");
        self.draw_legend();
        self.draw_svg_comments("plot data");
        self.plot_data();
        self.svg.finish()
    }

    fn plot_data(&mut self) {
        for series in &self.data {
            if let Some(first) = self.data.first() {
                assert_eq!(series.values.len(), first.values.len())
            }

            let n = self.layout.axis_count();

            let initial_points = self.layout.data_to_points(&vec![0.0; n]);
            // let initial_points = self.polygon_points(&vec![0.0; n]); // Points at center
            // let target_points = self.polygon_points(&series.values);
            let target_points = self.layout.data_to_points(&series.values);

            self.svg.push_raw(&format!(
    "<polygon points=\"{target_points}\" style=\"fill:{color};fill-opacity:0.25;stroke:{color};stroke-width:{stroke_width};\">
        <animate 
            attributeName=\"points\" 
            from=\"{initial_points}\" 
            to=\"{target_points}\" 
            dur=\"1.8s\" 
            begin=\"0s\" 
            fill=\"freeze\" 
            calcMode=\"spline\" 
            keyTimes=\"0;1\" 
            keySplines=\"0.4 0 0.2 1\" />
    </polygon>",
    target_points = target_points,
    initial_points = initial_points,
    color = series.color,
    stroke_width = self.theme.stroke_width,
));
        }
    }

    fn draw_bg(&mut self) {
        let half_size = self.layout.size / 2.0;
        let min_x = -half_size;
        let min_y = -half_size;

        self.svg.push_raw(&format!(
            "  <rect id=\"chart-bg\" x=\"{min_x}\" y=\"{min_y}\" width=\"100%\" height=\"100%\" fill=\"{}\" />\n",
            self.theme.bg.unwrap_or_default()
        ));
    }

    fn draw_grid_levels(&mut self, n: usize) {
        let n_edges = self.layout.axis_count();
        // 2. Grid Polygons (Concentric Webs)
        for level in 1..=n {
            let scale = level as f64 / n as f64;
            let scale_vec = vec![scale; n_edges];
            Polygon {
                points: self.layout.data_to_points(&scale_vec),
                style: format!(
                    "fill:none;stroke:{};stroke-opacity:0.2;stroke-width:{};",
                    self.theme.scale.unwrap_or_default(),
                    self.theme.stroke_width
                ),
            }
            .render(&mut self.svg);
        }
    }

    pub fn set_axes_labels(&mut self, axes_labels: Vec<String>) {
        self.layout.series_len = axes_labels.len();
        self.axes_labels = axes_labels;
    }
}
