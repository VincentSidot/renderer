//! Image module for the renderer

use crate::{backend::Backend, shape::Shape};
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
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the height of the image
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
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
