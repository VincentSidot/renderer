use crate::Color;
use crate::Stroke;

/// A rectangle shape
#[derive(Debug, Clone)]
pub struct Rectangle {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
    /// Width
    pub width: f32,
    /// Height
    pub height: f32,
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
    pub fn with_pos(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set the size of the rectangle
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
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
