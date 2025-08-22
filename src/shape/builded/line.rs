use crate::Stroke;
use std::convert::Into;

/// A line shape
#[derive(Debug, Clone)]
pub struct Line {
    /// Start X position
    pub(crate) x1: f64,
    /// Start Y position
    pub(crate) y1: f64,
    /// End X position
    pub(crate) x2: f64,
    /// End Y position
    pub(crate) y2: f64,
    /// Stroke
    pub(crate) stroke: Option<Stroke>,
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
        self.x1 = x.into();
        self.y1 = y.into();
        self
    }

    /// Set the end position of the line
    pub fn with_end<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.x2 = x.into();
        self.y2 = y.into();
        self
    }

    /// Set the stroke of the line
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }

    /// Get the start X position
    pub fn x1(&self) -> f64 {
        self.x1
    }

    /// Get the start Y position
    pub fn y1(&self) -> f64 {
        self.y1
    }

    /// Get the end X position
    pub fn x2(&self) -> f64 {
        self.x2
    }

    /// Get the end Y position
    pub fn y2(&self) -> f64 {
        self.y2
    }

    /// Get the stroke
    pub fn stroke(&self) -> Option<&Stroke> {
        self.stroke.as_ref()
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}