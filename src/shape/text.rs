use crate::Color;
use std::convert::Into;

/// A text shape
#[derive(Debug, Clone)]
pub struct Text {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
    /// Text content
    pub content: String,
    /// Font size
    pub font_size: f32,
    /// Fill color
    pub fill_color: Option<Color>,
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
        self.x = x.into() as f32;
        self.y = y.into() as f32;
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
        self.font_size = size.into() as f32;
        self
    }

    /// Set the fill color of the text
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}
