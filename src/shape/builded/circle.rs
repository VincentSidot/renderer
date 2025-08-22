use crate::Color;
use crate::Stroke;
use std::convert::Into;

/// A circle shape
#[derive(Debug, Clone)]
pub struct Circle {
    /// X position
    pub x: f64,
    /// Y position
    pub y: f64,
    /// Radius
    pub radius: f64,
    /// Fill color
    pub fill_color: Option<Color>,
    /// Stroke
    pub stroke: Option<Stroke>,
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
}

impl Default for Circle {
    fn default() -> Self {
        Self::new()
    }
}
