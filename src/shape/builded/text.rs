use crate::Color;
use std::convert::Into;

/// A text shape
#[derive(Debug, Clone)]
pub struct Text {
    /// X position
    pub(crate) x: f64,
    /// Y position
    pub(crate) y: f64,
    /// Text content
    pub(crate) content: String,
    /// Font size
    pub(crate) font_size: f64,
    /// Fill color
    pub(crate) fill_color: Option<Color>,
}

impl Text {
    /// Create a new text
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            content: String::new(),
            font_size: 16.0,
            fill_color: None,
        }
    }

    /// Set the position of the text
    pub fn with_pos<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.x = x.into();
        self.y = y.into();
        self
    }

    /// Set the text content
    pub fn with_text<S: Into<String>>(mut self, text: S) -> Self {
        self.content = text.into();
        self
    }

    /// Set the font size
    pub fn with_font_size<T>(mut self, size: T) -> Self
    where
        T: Into<f64>,
    {
        self.font_size = size.into();
        self
    }

    /// Set the fill color of the text
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
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

    /// Get the text content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Get the font size
    pub fn font_size(&self) -> f64 {
        self.font_size
    }

    /// Get the fill color
    pub fn fill_color(&self) -> Option<&Color> {
        self.fill_color.as_ref()
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}