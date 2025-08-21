use crate::Color;
use crate::Stroke;

/// A circle shape
#[derive(Debug, Clone)]
pub struct Circle {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
    /// Radius
    pub radius: f32,
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
    pub fn with_pos(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set the radius of the circle
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
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
