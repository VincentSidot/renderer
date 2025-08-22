use crate::Color;
use crate::Stroke;
use std::convert::Into;

/// A rectangle shape
#[derive(Debug, Clone)]
pub struct Rectangle {
    /// X position
    pub x: f64,
    /// Y position
    pub y: f64,
    /// Width
    pub width: f64,
    /// Height
    pub height: f64,
    /// Fill color
    pub fill_color: Option<Color>,
    /// Stroke
    pub stroke: Option<Stroke>,
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
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new()
    }
}