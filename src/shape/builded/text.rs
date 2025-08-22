use crate::Color;

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
    pub(crate) font_size: f32,
    /// Fill color
    pub(crate) fill_color: Option<Color>,
}

impl Text {
    /// Create a new text
    pub(crate) fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            content: String::new(),
            font_size: 16.0,
            fill_color: None,
        }
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
    pub fn font_size(&self) -> f32 {
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