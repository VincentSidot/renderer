//! Bezier curve rasterization implementation

use crate::{
    rasterizer::{Pixel, PixelImage},
    shape::BezierCurve,
};

impl super::Rasterizer for BezierCurve {
    fn rasterize(&self, image: &mut PixelImage) {
        // For now, we'll just draw lines between the control points
        // In a real implementation, you would rasterize the actual bezier curve
        if self.points.is_empty() {
            return;
        }

        // Use a simple color conversion for now
        let stroke = self.stroke.unwrap_or_default();
        let color = stroke.color;
        let pixel = Pixel::new(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        );

        // Draw lines between consecutive points
        for i in 0..self.points.len().saturating_sub(1) {
            let start = self.points[i];
            let end = self.points[i + 1];

            // Bresenham's line algorithm
            let x1 = start.0 as i32;
            let y1 = start.1 as i32;
            let x2 = end.0 as i32;
            let y2 = end.1 as i32;

            let dx = (x2 - x1).abs();
            let dy = (y2 - y1).abs();
            let sx = if x1 < x2 { 1 } else { -1 };
            let sy = if y1 < y2 { 1 } else { -1 };
            let mut err = dx - dy;

            let mut x = x1;
            let mut y = y1;

            loop {
                // Draw the pixel if it's within bounds
                if x >= 0 && x < image.width as i32 && y >= 0 && y < image.height as i32 {
                    image.set_pixel(x as usize, y as usize, pixel);
                }

                // Check if we've reached the end point
                if x == x2 && y == y2 {
                    break;
                }

                let e2 = 2 * err;
                if e2 > -dy {
                    err -= dy;
                    x += sx;
                }
                if e2 < dx {
                    err += dx;
                    y += sy;
                }
            }
        }
    }

    fn rasterize_filled(&self, image: &mut PixelImage) {
        // Bezier curves don't have a filled version, so we just call the regular rasterize
        self.rasterize(image);
    }
}