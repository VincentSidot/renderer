//! Binary PPM Backend

#[cfg(feature = "rasterizer")]
use crate::rasterizer::{PixelImage, Rasterizer};
use crate::shape::Shape;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct PPMBackend;

impl PPMBackend {
    /// Create a new binary PPM backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for PPMBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "rasterizer")]
impl super::Backend for PPMBackend {
    fn render(
        &mut self,
        image: &crate::Image,
        path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create a pixel image with the same dimensions as the image
        let mut pixel_image = PixelImage::new(image.width as usize, image.height as usize);

        // Rasterize each shape onto the pixel image
        for shape in &image.shapes {
            match shape {
                Shape::Rectangle(rect) => rect.rasterize_filled(&mut pixel_image),
                Shape::Circle(circle) => circle.rasterize_filled(&mut pixel_image),
                Shape::Line(line) => line.rasterize(&mut pixel_image),
                Shape::Text(_) => {
                    // Text rasterization is not implemented in this backend
                    // In a real implementation, we would need a font rendering system
                }
                Shape::Ellipse(ellipse) => ellipse.rasterize_filled(&mut pixel_image),
                Shape::Polygon(polygon) => polygon.rasterize_filled(&mut pixel_image),
            }
        }

        // Save the pixel image as a binary PPM file
        save_as_binary_ppm(&pixel_image, path)
    }
}

#[cfg(not(feature = "rasterizer"))]
impl super::Backend for PPMBackend {
    fn render(
        &mut self,
        _image: &crate::Image,
        _path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Err("Rasterizer feature is not enabled".into())
    }
}

/// Save the pixel image as a binary PPM image
#[cfg(feature = "rasterizer")]
fn save_as_binary_ppm(image: &PixelImage, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;

    // Write PPM header (P6 indicates binary format)
    writeln!(file, "P6")?;
    writeln!(file, "{} {}", image.width, image.height)?;
    writeln!(file, "255")?;

    // Write pixel data as binary
    for pixel in &image.pixels {
        file.write_all(&[pixel.r, pixel.g, pixel.b])?;
    }

    Ok(())
}
