//! Image module for the renderer

use crate::{backend::Backend, shape::Shape};
use std::convert::Into;
use std::path::Path;

/// Image representation
#[derive(Debug)]
pub struct Image {
    /// Width of the image
    pub width: f32,
    /// Height of the image
    pub height: f32,
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
        self.width = width.into() as f32;
        self
    }

    /// Set the height of the image
    pub fn with_height<T>(mut self, height: T) -> Self
    where
        T: Into<f64>,
    {
        self.height = height.into() as f32;
        self
    }

    /// Add a shape to the image
    pub fn add(&mut self, shape: Shape) {
        self.shapes.push(shape);
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
}
