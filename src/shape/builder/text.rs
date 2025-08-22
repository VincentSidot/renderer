//! Text shape builder

use crate::{
    builder::{BuildError, ShapeBuilder},
    shape::Text,
    Color,
};
use std::convert::Into;

/// Builder for creating text with validation
#[derive(Debug, Default)]
pub struct TextBuilder {
    x: Option<f64>,
    y: Option<f64>,
    content: Option<String>,
    font_size: Option<f32>,
    #[cfg(any(feature = "rasterizer", feature = "fonts"))]
    width: Option<f64>,
    #[cfg(any(feature = "rasterizer", feature = "fonts"))]
    height: Option<f64>,
    fill_color: Option<Color>,
}

impl TextBuilder {
    /// Create a new text builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the position of the text
    pub fn with_pos<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.x = Some(x.into());
        self.y = Some(y.into());
        self
    }

    /// Set the text content
    pub fn with_text<S: Into<String>>(mut self, text: S) -> Self {
        self.content = Some(text.into());
        self
    }

    /// Set the font size
    pub fn with_font_size<T>(mut self, size: T) -> Self
    where
        T: Into<f32>,
    {
        self.font_size = Some(size.into());
        self
    }

    /// Set the fill color of the text
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set the size of the box the text should fit in, calculating the best font size
    /// This feature requires the "rasterizer" or "fonts" feature
    #[cfg(any(feature = "rasterizer", feature = "fonts"))]
    pub fn with_size<T, U>(mut self, width: T, height: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.width = Some(width.into());
        self.height = Some(height.into());
        self
    }
}

impl ShapeBuilder<Text> for TextBuilder {
    /// Build the text, performing validation
    fn build(self) -> Result<Text, BuildError> {
        let x = self.x.unwrap_or(0.0);
        let y = self.y.unwrap_or(0.0);
        let content = self.content.ok_or_else(|| {
            BuildError::MissingRequiredField("Text content must be specified".to_string())
        })?;

        #[cfg(any(feature = "rasterizer", feature = "fonts"))]
        {
            // If width and height are specified, we need the rasterizer feature
            if self.width.is_some() || self.height.is_some() {
                #[cfg(not(any(feature = "rasterizer", feature = "fonts")))]
                {
                    return Err(BuildError::FeatureUnavailable(
                        "Font sizing requires the 'rasterizer' or 'fonts' feature".to_string(),
                    ));
                }

                #[cfg(any(feature = "rasterizer", feature = "fonts"))]
                {
                    // Both width and height must be specified if either is
                    let width = self.width.ok_or_else(|| {
                        BuildError::MissingRequiredField(
                            "Width must be specified when using with_size".to_string(),
                        )
                    })?;
                    let height = self.height.ok_or_else(|| {
                        BuildError::MissingRequiredField(
                            "Height must be specified when using with_size".to_string(),
                        )
                    })?;

                    // Validate width and height
                    if width.is_nan() || width <= 0.0 {
                        return Err(BuildError::InvalidValue(
                            "Width must be a positive number".to_string(),
                        ));
                    }

                    if height.is_nan() || height <= 0.0 {
                        return Err(BuildError::InvalidValue(
                            "Height must be a positive number".to_string(),
                        ));
                    }

                    // For now, we'll just use a simple heuristic for font size
                    // In a real implementation, this would use the rasterizer to calculate
                    // the optimal font size based on the text content and box dimensions
                    let font_size = (width.min(height) * 0.8).min(72.0) as f32; // Cap at 72pt

                    return Ok(Text {
                        x,
                        y,
                        content,
                        font_size,
                        fill_color: self.fill_color,
                    });
                }
            }
        }

        let font_size = self.font_size.unwrap_or(16.0f32);

        // Validate font size
        if font_size.is_nan() || font_size <= 0.0 {
            return Err(BuildError::InvalidValue(
                "Font size must be a positive number".to_string(),
            ));
        }

        Ok(Text {
            x,
            y,
            content,
            font_size,
            fill_color: self.fill_color,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_text_builder_success() {
        let text = TextBuilder::new()
            .with_pos(10.0, 20.0)
            .with_text("Hello, World!")
            .with_font_size(16.0)
            .build()
            .expect("Failed to build text");

        assert_eq!(text.x(), 10.0);
        assert_eq!(text.y(), 20.0);
        assert_eq!(text.content(), "Hello, World!");
        assert_eq!(text.font_size(), 16.0);
    }

    #[test]
    fn test_text_builder_defaults() {
        let text = TextBuilder::new()
            .with_text("Hello, World!")
            .build()
            .expect("Failed to build text");

        assert_eq!(text.x(), 0.0);
        assert_eq!(text.y(), 0.0);
        assert_eq!(text.content(), "Hello, World!");
        assert_eq!(text.font_size(), 16.0);
    }

    #[test]
    fn test_text_builder_with_fill_color() {
        let color = Color::RED;

        let text = TextBuilder::new()
            .with_text("Hello, World!")
            .with_fill_color(color)
            .build()
            .expect("Failed to build text");

        assert_eq!(text.fill_color(), Some(&color));
    }

    #[test]
    fn test_text_builder_missing_content() {
        let result = TextBuilder::new().build();

        assert!(matches!(
            result,
            Err(BuildError::MissingRequiredField(_))
        ));
    }

    #[test]
    fn test_text_builder_invalid_font_size() {
        let result = TextBuilder::new()
            .with_text("Hello, World!")
            .with_font_size(-1.0)
            .build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }

    #[test]
    fn test_text_builder_nan_font_size() {
        let result = TextBuilder::new()
            .with_text("Hello, World!")
            .with_font_size(f32::NAN)
            .build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }

    #[cfg(any(feature = "rasterizer", feature = "fonts"))]
    #[test]
    fn test_text_builder_with_size() {
        let text = TextBuilder::new()
            .with_pos(10.0, 20.0)
            .with_text("Hello, World!")
            .with_size(100.0, 50.0)
            .build()
            .expect("Failed to build text");

        assert_eq!(text.x(), 10.0);
        assert_eq!(text.y(), 20.0);
        assert_eq!(text.content(), "Hello, World!");
        // Font size should be calculated based on the smaller dimension (50.0 * 0.8 = 40.0)
        assert_eq!(text.font_size(), 40.0);
    }
}