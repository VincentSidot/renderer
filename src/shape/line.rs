use crate::Stroke;

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
    pub fn with_pos(mut self, x: f32, y: f32) -> Self {
        self.x1 = x;
        self.y1 = y;
        self
    }

    /// Set the end position of the line
    pub fn with_end(mut self, x: f32, y: f32) -> Self {
        self.x2 = x;
        self.y2 = y;
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
