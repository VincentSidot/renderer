//! PNG Backend

use crate::shape::Shape;
#[cfg(feature = "rasterizer")]
use crate::rasterizer::{FontRenderer, PixelImage, Rasterizer};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Debug)]
pub struct PNGBackend {
    #[cfg(feature = "rasterizer")]
    font_data: Option<Vec<u8>>,
}

impl PNGBackend {
    /// Create a new PNG backend
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "rasterizer")]
            font_data: None,
        }
    }
    
    /// Create a new PNG backend with custom font data
    #[cfg(feature = "rasterizer")]
    pub fn with_font_data(font_data: Vec<u8>) -> Self {
        Self {
            font_data: Some(font_data),
        }
    }
}

impl Default for PNGBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "rasterizer")]
impl super::Backend for PNGBackend {
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
                    // Rasterize the bezier curve directly
                    bezier.rasterize(&mut pixel_image);
                }
            }
        }

        // Save the pixel image as a PNG file
        save_as_png(&pixel_image, path)
    }
}

#[cfg(not(feature = "rasterizer"))]
impl super::Backend for PNGBackend {
    fn render(
        &mut self,
        _image: &crate::Image,
        _path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Err("Rasterizer feature is not enabled".into())
    }
}

/// Save the pixel image as a PNG image
#[cfg(feature = "rasterizer")]
fn save_as_png(image: &PixelImage, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image.width as u32, image.height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    // Convert our pixel data to the format expected by the PNG encoder
    let mut png_data = Vec::with_capacity(image.pixels.len() * 3);
    for pixel in &image.pixels {
        png_data.push(pixel.r);
        png_data.push(pixel.g);
        png_data.push(pixel.b);
    }

    writer.write_image_data(&png_data)?;

    Ok(())
}

#[cfg(test)]
#[cfg(feature = "png")]
mod tests {
    use crate::backend::PNGBackend;

    #[test]
    fn test_png_backend_creation() {
        let _backend = PNGBackend::new();
        assert!(true); // Just checking that creation works
    }

    #[test]
    fn test_png_backend_with_font_data() {
        let font_data = vec![0u8; 100]; // Dummy font data
        let _backend = PNGBackend::with_font_data(font_data);
        assert!(true); // Just checking that creation works
    }
}