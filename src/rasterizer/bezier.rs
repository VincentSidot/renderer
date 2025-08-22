//! Bezier curve rasterization implementation

use crate::{
    rasterizer::{Pixel, PixelImage},
    shape::BezierCurve,
};

impl super::Rasterizer for BezierCurve {
    fn rasterize(&self, image: &mut PixelImage) {
        if self.points.len() < 2 {
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

        // Rasterize based on the number of control points
        match self.points.len() {
            2 => {
                // Linear curve - draw a line between two points
                draw_line(
                    image,
                    self.points[0].0 as i32,
                    self.points[0].1 as i32,
                    self.points[1].0 as i32,
                    self.points[1].1 as i32,
                    pixel,
                );
            }
            3 => {
                // Quadratic bezier curve
                rasterize_quadratic_bezier(image, &self.points, pixel);
            }
            4 => {
                // Cubic bezier curve
                rasterize_cubic_bezier(image, &self.points, pixel);
            }
            _ => {
                // For more than 4 points, split into segments and rasterize each
                rasterize_high_order_bezier(image, &self.points, pixel);
            }
        }
    }

    fn rasterize_filled(&self, image: &mut PixelImage) {
        // Bezier curves don't have a filled version, so we just call the regular rasterize
        self.rasterize(image);
    }
}

/// Draw a line using Bresenham's algorithm
fn draw_line(image: &mut PixelImage, x1: i32, y1: i32, x2: i32, y2: i32, pixel: Pixel) {
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

/// Rasterize a quadratic bezier curve using de Casteljau's algorithm
fn rasterize_quadratic_bezier(image: &mut PixelImage, points: &[(f64, f64)], pixel: Pixel) {
    // Convert to a format that's easier to work with
    let p0 = points[0];
    let p1 = points[1];
    let p2 = points[2];
    
    // Sample the curve at regular intervals
    let steps = calculate_steps(p0, p1, p2);
    
    let mut prev_point = p0;
    
    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        
        // Quadratic bezier formula: B(t) = (1-t)²P₀ + 2(1-t)tP₁ + t²P₂
        let one_minus_t = 1.0 - t;
        let x = one_minus_t * one_minus_t * p0.0 + 
                2.0 * one_minus_t * t * p1.0 + 
                t * t * p2.0;
        let y = one_minus_t * one_minus_t * p0.1 + 
                2.0 * one_minus_t * t * p1.1 + 
                t * t * p2.1;
        
        // Draw a line from the previous point to the current point
        draw_line(
            image,
            prev_point.0 as i32,
            prev_point.1 as i32,
            x as i32,
            y as i32,
            pixel,
        );
        
        prev_point = (x, y);
    }
}

/// Rasterize a cubic bezier curve using de Casteljau's algorithm
fn rasterize_cubic_bezier(image: &mut PixelImage, points: &[(f64, f64)], pixel: Pixel) {
    // Convert to a format that's easier to work with
    let p0 = points[0];
    let p1 = points[1];
    let p2 = points[2];
    let p3 = points[3];
    
    // Sample the curve at regular intervals
    let steps = calculate_steps_cubic(p0, p1, p2, p3);
    
    let mut prev_point = p0;
    
    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        
        // Cubic bezier formula: B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
        let one_minus_t = 1.0 - t;
        let one_minus_t_sq = one_minus_t * one_minus_t;
        let t_sq = t * t;
        
        let x = one_minus_t_sq * one_minus_t * p0.0 + 
                3.0 * one_minus_t_sq * t * p1.0 + 
                3.0 * one_minus_t * t_sq * p2.0 + 
                t_sq * t * p3.0;
        let y = one_minus_t_sq * one_minus_t * p0.1 + 
                3.0 * one_minus_t_sq * t * p1.1 + 
                3.0 * one_minus_t * t_sq * p2.1 + 
                t_sq * t * p3.1;
        
        // Draw a line from the previous point to the current point
        draw_line(
            image,
            prev_point.0 as i32,
            prev_point.1 as i32,
            x as i32,
            y as i32,
            pixel,
        );
        
        prev_point = (x, y);
    }
}

/// Rasterize higher-order bezier curves by splitting them into cubic segments
fn rasterize_high_order_bezier(image: &mut PixelImage, points: &[(f64, f64)], pixel: Pixel) {
    // For now, we'll just draw lines between consecutive control points
    // A more sophisticated implementation would split the curve into cubic segments
    for i in 0..points.len() - 1 {
        draw_line(
            image,
            points[i].0 as i32,
            points[i].1 as i32,
            points[i + 1].0 as i32,
            points[i + 1].1 as i32,
            pixel,
        );
    }
}

/// Calculate the number of steps needed for a quadratic bezier curve
fn calculate_steps(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64)) -> usize {
    // Estimate the length of the curve by calculating the distance between control points
    let len1 = ((p1.0 - p0.0).powi(2) + (p1.1 - p0.1).powi(2)).sqrt();
    let len2 = ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt();
    let total_len = len1 + len2;
    
    // Use a simple heuristic: at least 10 steps, plus more steps for longer curves
    (10.0 + total_len / 5.0).max(2.0) as usize
}

/// Calculate the number of steps needed for a cubic bezier curve
fn calculate_steps_cubic(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> usize {
    // Estimate the length of the curve by calculating the distance between control points
    let len1 = ((p1.0 - p0.0).powi(2) + (p1.1 - p0.1).powi(2)).sqrt();
    let len2 = ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt();
    let len3 = ((p3.0 - p2.0).powi(2) + (p3.1 - p2.1).powi(2)).sqrt();
    let total_len = len1 + len2 + len3;
    
    // Use a simple heuristic: at least 10 steps, plus more steps for longer curves
    (10.0 + total_len / 5.0).max(2.0) as usize
}