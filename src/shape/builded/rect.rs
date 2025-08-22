use crate::Color;
use crate::Stroke;
use std::convert::Into;

/// A rectangle shape
#[derive(Debug, Clone)]
pub struct Rectangle {
    /// X position
    pub(crate) x: f64,
    /// Y position
    pub(crate) y: f64,
    /// Width
    pub(crate) width: f64,
    /// Height
    pub(crate) height: f64,
    /// Fill color
    pub(crate) fill_color: Option<Color>,
    /// Stroke
    pub(crate) stroke: Option<Stroke>,
}

impl Rectangle {
    /// Create a new rectangle
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            fill_color: None,
            stroke: None,
        }
    }

    /// Set the position of the rectangle
    pub fn with_pos<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.x = x.into();
        self.y = y.into();
        self
    }

    /// Set the size of the rectangle
    pub fn with_size<T, U>(mut self, width: T, height: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.width = width.into();
        self.height = height.into();
        self
    }

    /// Set the fill color of the rectangle
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set the stroke of the rectangle
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

    /// Get the width
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Get the height
    pub fn height(&self) -> f64 {
        self.height
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

impl Default for Rectangle {
    fn default() -> Self {
        Self::new()
    }
}