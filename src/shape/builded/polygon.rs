use crate::Color;
use crate::Stroke;
use std::convert::Into;

/// A polygon shape
#[derive(Debug, Clone)]
pub struct Polygon {
    /// Points of the polygon (x, y)
    pub(crate) points: Vec<(f64, f64)>,
    /// Fill color
    pub(crate) fill_color: Option<Color>,
    /// Stroke
    pub(crate) stroke: Option<Stroke>,
}

impl Polygon {
    /// Create a new polygon
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            fill_color: None,
            stroke: None,
        }
    }

    /// Add a point to the polygon
    pub fn add_point<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.points.push((x.into(), y.into()));
        self
    }

    /// Set multiple points for the polygon
    pub fn with_points(mut self, points: Vec<(f64, f64)>) -> Self {
        self.points = points;
        self
    }

    /// Set the fill color of the polygon
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set the stroke of the polygon
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }

    /// Get the points of the polygon
    pub fn points(&self) -> &[(f64, f64)] {
        &self.points
    }

    /// Get the fill color
    pub fn fill_color(&self) -> Option<&Color> {
        self.fill_color.as_ref()
    }

    /// Get the stroke
    pub fn stroke(&self) -> Option<&Stroke> {
        self.stroke.as_ref()
    }
}

impl Default for Polygon {
    fn default() -> Self {
        Self::new()
    }
}