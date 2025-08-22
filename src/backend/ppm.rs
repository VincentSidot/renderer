//! Binary PPM Backend

use crate::shape::Shape;
#[cfg(feature = "rasterizer")]
use crate::rasterizer::{FontRenderer, PixelImage, Rasterizer};
use crate::builder::ShapeBuilder;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct PPMBackend {
    #[cfg(feature = "rasterizer")]
    font_data: Option<Vec<u8>>,
}

impl PPMBackend {
    /// Create a new binary PPM backend
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "rasterizer")]
            font_data: None,
        }
    }
    
    /// Create a new binary PPM backend with custom font data
    #[cfg(feature = "rasterizer")]
    pub fn with_font_data(font_data: Vec<u8>) -> Self {
        Self {
            font_data: Some(font_data),
        }
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
                Shape::Text(text) => {
                    // Render text with the font renderer
                    let color = text.fill_color.unwrap_or(crate::Color::BLACK);
                    let pixel_color = crate::rasterizer::Pixel::new(
                        (color.r * 255.0) as u8,
                        (color.g * 255.0) as u8,
                        (color.b * 255.0) as u8,
                        (color.a * 255.0) as u8,
                    );
                    // Create a font renderer with the text's font size
                    let text_font_renderer = if let Some(ref font_data) = self.font_data {
                        FontRenderer::new(Some(font_data.as_slice()), text.font_size)?
                    } else {
                        FontRenderer::default(text.font_size)?
                    };
                    let _ = text_font_renderer.rasterize_text(
                        &mut pixel_image,
                        &text.content,
                        text.x as usize,
                        text.y as usize,
                        pixel_color,
                    );
                }
                Shape::Ellipse(ellipse) => ellipse.rasterize_filled(&mut pixel_image),
                Shape::Polygon(polygon) => polygon.rasterize_filled(&mut pixel_image),
                Shape::BezierCurve(bezier) => {
                    // For now, we'll just draw lines between the control points
                    // In a real implementation, you would rasterize the actual bezier curve
                    if bezier.points.len() >= 2 {
                        for i in 0..bezier.points.len() - 1 {
                            let start = bezier.points[i];
                            let end = bezier.points[i + 1];
                            
                            // Create a temporary line and rasterize it
                            let line = crate::shape::LineBuilder::new()
                                .with_start(start.0, start.1)
                                .with_end(end.0, end.1)
                                .with_stroke(bezier.stroke.unwrap_or_default())
                                .build()
                                .unwrap(); // This should be safe since we've validated the stroke
                                
                            line.rasterize(&mut pixel_image);
                        }
                    }
                }
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
