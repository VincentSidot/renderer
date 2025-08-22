use crate::Color;
use crate::Stroke;
use std::convert::Into;

/// An ellipse shape
#[derive(Debug, Clone)]
pub struct Ellipse {
    /// X position
    pub(crate) x: f64,
    /// Y position
    pub(crate) y: f64,
    /// Radius along the x-axis
    pub(crate) radius_x: f64,
    /// Radius along the y-axis
    pub(crate) radius_y: f64,
    /// Fill color
    pub(crate) fill_color: Option<Color>,
    /// Stroke
    pub(crate) stroke: Option<Stroke>,
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
    pub fn with_pos<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.x = x.into();
        self.y = y.into();
        self
    }

    /// Set the radii of the ellipse
    pub fn with_radii<T, U>(mut self, radius_x: T, radius_y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.radius_x = radius_x.into();
        self.radius_y = radius_y.into();
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

    /// Get the X position
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Get the Y position
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Get the radius along the x-axis
    pub fn radius_x(&self) -> f64 {
        self.radius_x
    }

    /// Get the radius along the y-axis
    pub fn radius_y(&self) -> f64 {
        self.radius_y
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

impl Default for Ellipse {
    fn default() -> Self {
        Self::new()
    }
}