use crate::Color;
use crate::Stroke;
use std::convert::Into;

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
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            radius: 0.0,
            fill_color: None,
            stroke: None,
        }
    }

    /// Set the position of the circle
    pub fn with_pos<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.x = x.into();
        self.y = y.into();
        self
    }

    /// Set the radius of the circle
    pub fn with_radius<T>(mut self, radius: T) -> Self
    where
        T: Into<f64>,
    {
        self.radius = radius.into();
        self
    }

    /// Set the fill color of the circle
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set the stroke of the circle
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
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
