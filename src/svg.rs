#[derive(Clone)]
pub struct Svg(String);

pub type StyleElm<'a> = (&'a str, &'a str);

impl AsRef<[u8]> for Svg {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

// "<line x1="{sx}" y1="{sy}" x2="{ex}" y2="{ey}" stroke="#aaa"/>"
pub struct Line {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub style: String,
}

impl RenderSvg for Line {
    fn render(&self, svg: &mut Svg) {
        writeln!(
            svg,
            "  <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" style=\"{}\" />",
            self.x1, self.y1, self.x2, self.y2, self.style
        )
        .unwrap();
    }
}

pub struct Circle<'a> {
    pub r: f64,
    pub cx: f64,
    pub cy: f64,
    pub attrs: HashMap<&'a str, &'a str>,
}

// common, later to be used as blanket impl or via impl macro
impl<'a> Circle<'a> {
    pub fn attr(mut self, key: &'a str, val: &'a str) -> Self {
        self.attrs.insert(key, val);
        self
    }
}

impl RenderSvg for Circle<'_> {
    fn render(&self, svg: &mut Svg) {
        let attrs = self
            .attrs
            .iter()
            .map(|(key, value)| format!("{}:{};", key, value))
            .collect::<Vec<_>>()
            .join("");
        writeln!(
            svg,
            r#"  <circle cx="{}" cy="{}" r="{}" style="{}" />"#,
            self.cx, self.cy, self.r, attrs
        )
        .unwrap();
    }
}

pub struct Text {
    pub x: f64,
    pub y: f64,
    pub content: String,
    pub style: String,
}

impl RenderSvg for Text {
    fn render(&self, svg: &mut Svg) {
        writeln!(
            svg,
            "  <text x=\"{:.2}\" y=\"{:.2}\" style=\"{}\" font-family=\"sans-serif\">{}</text>",
            self.x, self.y, self.style, self.content
        )
        .unwrap();
    }
}

pub struct Polygon {
    pub points: String,
    pub style: String,
}

pub struct PolygonBuilder {
    pub points: String,
    pub style: String,
}

impl Polygon {
    // where:
    //  value = 0.0..1.0
    pub fn points_from_value(&self, _values: &[f64], attributes: &str) -> String {
        let points = "w";
        format!(r#"  <polygon points="{}" {} />"#, points, attributes)
    }
}

impl RenderSvg for Polygon {
    fn render(&self, svg: &mut Svg) {
        writeln!(
            svg,
            "  <polygon points=\"{p}\" style=\"{s}\" />",
            p = self.points,
            s = self.style,
        )
        .unwrap();
    }
}

impl std::fmt::Display for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

use std::collections::HashMap;
use std::fmt::Write;

impl std::fmt::Write for Svg {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.push_str(s);
        Ok(())
    }
}

pub trait RenderSvg {
    fn render(&self, svg: &mut Svg);
}

impl Svg {
    pub fn new(min_w: f64, min_h: f64, h: f64, w: f64) -> Self {
        Self(format!(
            "<svg width=\"{w}\" height=\"{h}\" viewBox=\"{min_w} {min_h} {w} {h}\" xmlns=\"http://www.w3.org/2000/svg\">\n"
        ))
    }

    pub fn push_raw(&mut self, string: &str) -> &mut Self {
        self.0.push_str(string);
        self
    }

    pub fn finish(&mut self) -> String {
        self.push_raw("</svg>");
        std::mem::take(&mut self.0)
    }
}
