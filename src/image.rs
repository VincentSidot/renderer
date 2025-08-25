//! Image module for the renderer

use crate::{backend::Backend, builder::BuildError, shape::Shape};
use std::convert::Into;
use std::path::Path;

/// Image representation
#[derive(Debug)]
pub struct Image {
    /// Width of the image
    pub width: f64,
    /// Height of the image
    pub height: f64,
    /// Shapes in the image
    pub shapes: Vec<Shape>,
}

impl Image {
    /// Create a new image
    pub fn new() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
            shapes: Vec::new(),
        }
    }

    /// Set the width of the image
    pub fn with_width<T>(mut self, width: T) -> Self
    where
        T: Into<f64>,
    {
        self.width = width.into();
        self
    }

    /// Set the height of the image
    pub fn with_height<T>(mut self, height: T) -> Self
    where
        T: Into<f64>,
    {
        self.height = height.into();
        self
    }

    /// Add a shape to the image
    pub fn add(&mut self, shape: impl Into<Shape>) {
        self.shapes.push(shape.into());
    }

    /// Save the image using the provided backend
    pub fn save<P: AsRef<Path>, B: Backend>(
        &self,
        path: P,
        backend: &mut B,
    ) -> Result<(), Box<dyn std::error::Error>> {
        backend.render(self, path.as_ref())
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating images with validation
#[derive(Debug)]
pub struct ImageBuilder {
    width: Option<f64>,
    height: Option<f64>,
    shapes: Vec<Shape>,
}

impl ImageBuilder {
    /// Create a new image builder
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            shapes: Vec::new(),
        }
    }

    /// Set the width of the image
    pub fn with_width<T>(mut self, width: T) -> Self
    where
        T: Into<f64>,
    {
        self.width = Some(width.into());
        self
    }

    /// Set the height of the image
    pub fn with_height<T>(mut self, height: T) -> Self
    where
        T: Into<f64>,
    {
        self.height = Some(height.into());
        self
    }

    /// Add a pre-built shape to the image
    pub fn add_shape(mut self, shape: Shape) -> Self {
        self.shapes.push(shape);
        self
    }

    /// Build the image, performing validation
    pub fn build(self) -> Result<Image, BuildError> {
        let width = self.width.unwrap_or(800.0);
        let height = self.height.unwrap_or(600.0);

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

        Ok(Image {
            width,
            height,
            shapes: self.shapes,
        })
    }
}

impl Default for ImageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_with_different_numeric_types() {
        // Test with integers
        let image1 = Image::new().with_width(800i32).with_height(600i32);
        assert_eq!(image1.width, 800.0);
        assert_eq!(image1.height, 600.0);

        // Test with floats
        let image2 = Image::new().with_width(800.5f64).with_height(600.7f64);
        assert_eq!(image2.width, 800.5);
        assert_eq!(image2.height, 600.7);

        // Test with mixed types
        let image3 = Image::new().with_width(800u32).with_height(600.7f64);
        assert_eq!(image3.width, 800.0);
        assert_eq!(image3.height, 600.7);

        // Test with different integer types that can be converted to f64
        let image4 = Image::new().with_width(800i16).with_height(600u32);
        assert_eq!(image4.width, 800.0);
        assert_eq!(image4.height, 600.0);
    }

    #[test]
    fn test_image_builder_success() {
        let image = ImageBuilder::new()
            .with_width(800.0)
            .with_height(600.0)
            .build()
            .expect("Failed to build image");

        assert_eq!(image.width, 800.0);
        assert_eq!(image.height, 600.0);
    }

    #[test]
    fn test_image_builder_defaults() {
        let image = ImageBuilder::new().build().expect("Failed to build image");

        assert_eq!(image.width, 800.0);
        assert_eq!(image.height, 600.0);
    }

    #[test]
    fn test_image_builder_invalid_width() {
        let result = ImageBuilder::new().with_width(-1.0).build();
        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }

    #[test]
    fn test_image_builder_invalid_height() {
        let result = ImageBuilder::new().with_height(0.0).build();
        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }

    #[test]
    fn test_image_builder_nan_values() {
        let result = ImageBuilder::new().with_width(f64::NAN).build();
        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }
}
