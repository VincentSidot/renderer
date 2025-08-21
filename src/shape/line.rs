use crate::Stroke;
use std::convert::Into;

/// A line shape
#[derive(Debug, Clone)]
pub struct Line {
    /// Start X position
    pub x1: f32,
    /// Start Y position
    pub y1: f32,
    /// End X position
    pub x2: f32,
    /// End Y position
    pub y2: f32,
    /// Stroke
    pub stroke: Option<Stroke>,
}

impl Line {
    /// Create a new line
    pub fn new() -> Self {
        Self {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
            stroke: None,
        }
    }

    /// Set the start position of the line
    pub fn with_pos<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.x1 = x.into() as f32;
        self.y1 = y.into() as f32;
        self
    }

    /// Set the end position of the line
    pub fn with_end<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.x2 = x.into() as f32;
        self.y2 = y.into() as f32;
        self
    }

    /// Set the stroke of the line
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}
