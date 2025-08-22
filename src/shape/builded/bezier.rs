use crate::Color;
use crate::Stroke;

/// A bezier curve shape defined by control points
#[derive(Debug, Clone)]
pub struct BezierCurve {
    /// Control points (x, y) - minimum 2 points for a linear curve, 3 for quadratic, 4 for cubic
    pub(crate) points: Vec<(f64, f64)>,
    /// Stroke
    pub(crate) stroke: Option<Stroke>,
    /// Fill color (for closed bezier curves)
    pub(crate) fill_color: Option<Color>,
}

impl BezierCurve {
    /// Create a new bezier curve
    pub(crate) fn new() -> Self {
        Self {
            points: Vec::new(),
            stroke: None,
            fill_color: None,
        }
    }

    /// Get the control points
    pub fn points(&self) -> &[(f64, f64)] {
        &self.points
    }

    /// Get the stroke
    pub fn stroke(&self) -> Option<&Stroke> {
        self.stroke.as_ref()
    }

    /// Get the fill color
    pub fn fill_color(&self) -> Option<&Color> {
        self.fill_color.as_ref()
    }
}

impl Default for BezierCurve {
    fn default() -> Self {
        Self::new()
    }
}