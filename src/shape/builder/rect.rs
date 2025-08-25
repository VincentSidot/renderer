//! Rectangle shape builder

use crate::{
    Color, Stroke,
    builder::{BuildError, ShapeBuilder},
    shape::Rectangle,
};
use std::convert::Into;

/// Builder for creating rectangles with validation
#[derive(Debug, Default)]
pub struct RectangleBuilder {
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
    fill_color: Option<Color>,
    stroke: Option<Stroke>,
}

impl RectangleBuilder {
    /// Create a new rectangle builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the position of the rectangle
    pub fn with_pos<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.x = Some(x.into());
        self.y = Some(y.into());
        self
    }

    /// Set the size of the rectangle
    pub fn with_size<T, U>(mut self, width: T, height: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.width = Some(width.into());
        self.height = Some(height.into());
        self
    }

    /// Set the fill color of the rectangle
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set the stroke of the rectangle
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

impl ShapeBuilder<Rectangle> for RectangleBuilder {
    /// Build the rectangle, performing validation
    fn build(self) -> Result<Rectangle, BuildError> {
        let x = self.x.unwrap_or(0.0);
        let y = self.y.unwrap_or(0.0);
        let width = self.width.ok_or_else(|| {
            BuildError::MissingRequiredField("Rectangle width must be specified".to_string())
        })?;
        let height = self.height.ok_or_else(|| {
            BuildError::MissingRequiredField("Rectangle height must be specified".to_string())
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

        Ok(Rectangle {
            x,
            y,
            width,
            height,
            fill_color: self.fill_color,
            stroke: self.stroke,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color::Color, stroke::Stroke};

    #[test]
    fn test_rectangle_builder_success() {
        let rectangle = RectangleBuilder::new()
            .with_pos(10.0, 20.0)
            .with_size(50.0, 30.0)
            .build()
            .expect("Failed to build rectangle");

        assert_eq!(rectangle.x(), 10.0);
        assert_eq!(rectangle.y(), 20.0);
        assert_eq!(rectangle.width(), 50.0);
        assert_eq!(rectangle.height(), 30.0);
    }

    #[test]
    fn test_rectangle_builder_defaults() {
        let rectangle = RectangleBuilder::new()
            .with_size(50.0, 30.0)
            .build()
            .expect("Failed to build rectangle");

        assert_eq!(rectangle.x(), 0.0);
        assert_eq!(rectangle.y(), 0.0);
        assert_eq!(rectangle.width(), 50.0);
        assert_eq!(rectangle.height(), 30.0);
    }

    #[test]
    fn test_rectangle_builder_with_fill_and_stroke() {
        let color = Color::RED;
        let stroke = Stroke::new().with_size(2.0).with_color(Color::BLUE);

        let rectangle = RectangleBuilder::new()
            .with_size(50.0, 30.0)
            .with_fill_color(color)
            .with_stroke(stroke)
            .build()
            .expect("Failed to build rectangle");

        assert_eq!(rectangle.fill_color(), Some(&color));
        assert_eq!(rectangle.stroke(), Some(&stroke));
    }

    #[test]
    fn test_rectangle_builder_missing_width() {
        let result = RectangleBuilder::new().with_size(50.0, 30.0).build();
        // This should actually succeed, let's create a test for missing both
        assert!(result.is_ok());

        // Test with missing width
        let result = RectangleBuilder::new().with_pos(10.0, 20.0).build();
        assert!(matches!(result, Err(BuildError::MissingRequiredField(_))));
    }

    #[test]
    fn test_rectangle_builder_invalid_width() {
        let result = RectangleBuilder::new().with_size(-1.0, 30.0).build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }

    #[test]
    fn test_rectangle_builder_invalid_height() {
        let result = RectangleBuilder::new().with_size(50.0, 0.0).build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }

    #[test]
    fn test_rectangle_builder_nan_values() {
        let result = RectangleBuilder::new().with_size(f64::NAN, 30.0).build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }
}
