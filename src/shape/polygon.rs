use crate::Color;
use crate::Stroke;

/// A polygon shape
#[derive(Debug, Clone)]
pub struct Polygon {
    /// Points of the polygon (x, y)
    pub points: Vec<(f32, f32)>,
    /// Fill color
    pub fill_color: Option<Color>,
    /// Stroke
    pub stroke: Option<Stroke>,
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
    pub fn add_point(mut self, x: f32, y: f32) -> Self {
        self.points.push((x, y));
        self
    }

    /// Set multiple points for the polygon
    pub fn with_points(mut self, points: Vec<(f32, f32)>) -> Self {
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
}

impl Default for Polygon {
    fn default() -> Self {
        Self::new()
    }
}
