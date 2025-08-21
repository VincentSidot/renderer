use crate::Color;
use crate::Stroke;

/// An ellipse shape
#[derive(Debug, Clone)]
pub struct Ellipse {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
    /// Radius along the x-axis
    pub radius_x: f32,
    /// Radius along the y-axis
    pub radius_y: f32,
    /// Fill color
    pub fill_color: Option<Color>,
    /// Stroke
    pub stroke: Option<Stroke>,
}

impl Ellipse {
    /// Create a new ellipse
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            radius_x: 0.0,
            radius_y: 0.0,
            fill_color: None,
            stroke: None,
        }
    }

    /// Set the position of the ellipse
    pub fn with_pos(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set the radii of the ellipse
    pub fn with_radii(mut self, radius_x: f32, radius_y: f32) -> Self {
        self.radius_x = radius_x;
        self.radius_y = radius_y;
        self
    }

    /// Set the fill color of the ellipse
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set the stroke of the ellipse
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

impl Default for Ellipse {
    fn default() -> Self {
        Self::new()
    }
}
