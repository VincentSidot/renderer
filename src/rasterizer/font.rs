//! Font rasterization using ab_glyph

#[cfg(feature = "rasterizer")]
use crate::rasterizer::{Pixel, PixelImage};
#[cfg(feature = "rasterizer")]
use ab_glyph::{Font, FontRef, PxScale, ScaleFont};

/// Include the default Ubuntu font as bytes
#[cfg(feature = "rasterizer")]
const DEFAULT_FONT: &[u8] = include_bytes!("../../font/ubuntu.ttf");

/// A font that can be used to render text
#[derive(Debug)]
pub struct FontRenderer<'f> {
    #[cfg(feature = "rasterizer")]
    font: FontRef<'f>,
    #[cfg(feature = "rasterizer")]
    scale: PxScale,
}

impl<'f> FontRenderer<'f> {
    /// Create a new font renderer with the specified font data and size
    #[cfg(feature = "rasterizer")]
    pub fn new(font_data: Option<&'f [u8]>, size: f32) -> Result<Self, Box<dyn std::error::Error>> {
        let font_data = font_data.unwrap_or(DEFAULT_FONT);
        let font = FontRef::try_from_slice(font_data)?;
        let scale = PxScale::from(size);
        Ok(Self { font, scale })
    }

    /// Create a new font renderer with a default font (Ubuntu) and size
    #[cfg(feature = "rasterizer")]
    pub fn default(size: f32) -> Result<Self, Box<dyn std::error::Error>> {
        Self::new(None, size)
    }

    /// Rasterize text onto an image at the specified position
    #[cfg(feature = "rasterizer")]
    pub fn rasterize_text(
        &self,
        image: &mut PixelImage,
        text: &str,
        x: usize,
        y: usize,
        color: Pixel,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let scaled_font = self.font.as_scaled(self.scale);
        let mut caret_x = x as f32;
        let mut caret_y = y as f32;
        let line_height = scaled_font.height();

        for c in text.chars() {
            if c == '\n' {
                // Handle newlines
                caret_x = x as f32;
                caret_y += line_height; // Move down by line height
                // In a real implementation, you'd track the y position properly
                continue;
            } else if c.is_whitespace() {
                // Handle spaces by advancing the caret position
                caret_x += scaled_font.h_advance(self.font.glyph_id(c));
                continue;
            }

            let glyph_id = self.font.glyph_id(c);
            let glyph = glyph_id.with_scale(self.scale);

            if let Some(outline) = self.font.outline_glyph(glyph) {
                let bounds = outline.px_bounds();
                let glyph_width = bounds.width() as usize;
                let glyph_height = bounds.height() as usize;
                // Position the glyph at the correct location
                let glyph_x = (caret_x + bounds.min.x) as usize;
                let glyph_y = (caret_y + bounds.min.y) as usize;

                // Create a buffer to rasterize the glyph
                let mut glyph_buffer = vec![0u8; glyph_width * glyph_height];

                // Rasterize the glyph into the buffer
                outline.draw(|x, y, c| {
                    let idx = (y as usize) * glyph_width + (x as usize);
                    if idx < glyph_buffer.len() {
                        glyph_buffer[idx] = (c * 255.0) as u8;
                    }
                });

                // Copy the glyph buffer to the image
                for gy in 0..glyph_height {
                    for gx in 0..glyph_width {
                        let idx = gy * glyph_width + gx;
                        let alpha = glyph_buffer[idx];

                        if alpha > 0 {
                            let px = glyph_x + gx;
                            let py = glyph_y + gy;

                            if px < image.width && py < image.height {
                                // Blend the pixel with the existing pixel based on alpha
                                let existing =
                                    image.get_pixel(px, py).unwrap_or(&Pixel::TRANSPARENT);
                                let new_pixel = Pixel::new(
                                    (color.r as f32 * (alpha as f32 / 255.0)
                                        + existing.r as f32 * (1.0 - (alpha as f32 / 255.0)))
                                        as u8,
                                    (color.g as f32 * (alpha as f32 / 255.0)
                                        + existing.g as f32 * (1.0 - (alpha as f32 / 255.0)))
                                        as u8,
                                    (color.b as f32 * (alpha as f32 / 255.0)
                                        + existing.b as f32 * (1.0 - (alpha as f32 / 255.0)))
                                        as u8,
                                    255, // For simplicity, we're setting alpha to 255
                                );
                                image.set_pixel(px, py, new_pixel);
                            }
                        }
                    }
                }

                // Advance the caret position
                caret_x += scaled_font.h_advance(glyph_id);
            }
        }

        Ok(())
    }

    /// Calculate the width and height of the text when rendered
    #[cfg(feature = "rasterizer")]
    pub fn measure_text(&self, text: &str) -> (f32, f32) {
        let scaled_font = self.font.as_scaled(self.scale);
        let line_height = scaled_font.height();

        let mut width = 0.0f32;
        let mut max_width = 0.0f32;
        let mut lines = 1usize;

        for c in text.chars() {
            if c == '\n' {
                lines += 1;
                max_width = max_width.max(width);
                width = 0.0;
                continue;
            }

            let glyph_id = self.font.glyph_id(c);
            width += scaled_font.h_advance(glyph_id);
        }

        max_width = max_width.max(width);
        (max_width, lines as f32 * line_height)
    }
}

#[cfg(not(feature = "rasterizer"))]
impl FontRenderer {
    /// Create a new font renderer with the specified font data and size
    pub fn new(
        _font_data: Option<&'f [u8]>,
        _size: f32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Err("Rasterizer feature is not enabled".into())
    }

    /// Create a new font renderer with a default font (Ubuntu) and size
    pub fn default(_size: f32) -> Result<Self, Box<dyn std::error::Error>> {
        Err("Rasterizer feature is not enabled".into())
    }

    /// Rasterize text onto an image at the specified position
    pub fn rasterize_text(
        &self,
        _image: &mut PixelImage,
        _text: &str,
        _x: usize,
        _y: usize,
        _color: Pixel,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Err("Rasterizer feature is not enabled".into())
    }

    /// Calculate the width and height of the text when rendered
    pub fn measure_text(&self, _text: &str) -> (f32, f32) {
        (0.0, 0.0)
    }
}
