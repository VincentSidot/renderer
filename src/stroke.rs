//! Stroke module for the renderer

use crate::color::Color;

/// Stroke representation for shapes
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stroke {
    /// Stroke width
    pub width: f32,
    /// Stroke color
    pub color: Color,
}

impl Stroke {
    /// Create a new stroke with default values
    pub fn new() -> Self {
        Self {
            width: 1.0,
            color: Color::TRANSPARENT, // Default to transparent stroke
        }
    }

    /// Set the stroke width
    pub fn with_size(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the stroke color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Default for Stroke {
    fn default() -> Self {
        Self::new()
    }
}
