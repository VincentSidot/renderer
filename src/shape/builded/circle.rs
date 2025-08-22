use crate::Color;
use crate::Stroke;

/// A circle shape
#[derive(Debug, Clone)]
pub struct Circle {
    /// X position
    pub(crate) x: f64,
    /// Y position
    pub(crate) y: f64,
    /// Radius
    pub(crate) radius: f64,
    /// Fill color
    pub(crate) fill_color: Option<Color>,
    /// Stroke
    pub(crate) stroke: Option<Stroke>,
}

impl Circle {
    /// Create a new circle
    pub(crate) fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            radius: 0.0,
            fill_color: None,
            stroke: None,
        }
    }

    /// Get the X position
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Get the Y position
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Get the radius
    pub fn radius(&self) -> f64 {
        self.radius
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

impl Default for Circle {
    fn default() -> Self {
        Self::new()
    }
}
