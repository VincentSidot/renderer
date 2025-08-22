use crate::Color;
use crate::Stroke;

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
    pub(crate) fn new() -> Self {
        Self {
            points: Vec::new(),
            fill_color: None,
            stroke: None,
        }
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