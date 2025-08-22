mod bezier;
mod font;

pub use font::FontRenderer;

use crate::shape::{BezierCurve, Circle, Ellipse, Line, Polygon, Rectangle};

/// A simple RGBA pixel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    /// Create a new pixel with the specified RGBA values
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    /// Create a new pixel with RGB values (alpha defaults to 255)
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

/// Predefined colors
impl Pixel {
    /// Black pixel
    pub const BLACK: Pixel = Pixel { r: 0, g: 0, b: 0, a: 255 };
    
    /// White pixel
    pub const WHITE: Pixel = Pixel { r: 255, g: 255, b: 255, a: 255 };
    
    /// Transparent pixel
    pub const TRANSPARENT: Pixel = Pixel { r: 0, g: 0, b: 0, a: 0 };
}

/// A 2D raster image buffer with RGBA pixels
#[derive(Debug)]
pub struct PixelImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pixel>,
}

impl PixelImage {
    /// Create a new pixel image with the specified dimensions
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Pixel::WHITE; width * height],
        }
    }
    
    /// Create a new pixel image with the specified dimensions and background color
    pub fn with_background(width: usize, height: usize, background: Pixel) -> Self {
        Self {
            width,
            height,
            pixels: vec![background; width * height],
        }
    }
    
    /// Get a mutable reference to a pixel at the specified coordinates
    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        if x < self.width && y < self.height {
            Some(&mut self.pixels[y * self.width + x])
        } else {
            None
        }
    }
    
    /// Get a reference to a pixel at the specified coordinates
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&Pixel> {
        if x < self.width && y < self.height {
            Some(&self.pixels[y * self.width + x])
        } else {
            None
        }
    }
    
    /// Set a pixel at the specified coordinates
    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        if let Some(p) = self.get_pixel_mut(x, y) {
            *p = pixel;
        }
    }
    
    /// Fill the image with a specific color
    pub fn fill(&mut self, color: Pixel) {
        self.pixels.fill(color);
    }
    
    /// Blend a pixel with the existing pixel at the specified coordinates
    pub fn blend_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        if let Some(p) = self.get_pixel_mut(x, y) {
            // Simple alpha blending
            let alpha = pixel.a as f32 / 255.0;
            let inv_alpha = 1.0 - alpha;
            
            p.r = (pixel.r as f32 * alpha + p.r as f32 * inv_alpha) as u8;
            p.g = (pixel.g as f32 * alpha + p.g as f32 * inv_alpha) as u8;
            p.b = (pixel.b as f32 * alpha + p.b as f32 * inv_alpha) as u8;
            // For simplicity, we're not blending the alpha channel itself
        }
    }
}

/// Trait for rasterizing shapes to a pixel image
pub trait Rasterizer {
    /// Rasterize the shape onto the provided image
    fn rasterize(&self, image: &mut PixelImage);
    
    /// Rasterize the filled shape onto the provided image
    fn rasterize_filled(&self, image: &mut PixelImage);
}

impl Rasterizer for Rectangle {
    fn rasterize(&self, image: &mut PixelImage) {
        let x1 = self.x.max(0.0).min(image.width as f64 - 1.0) as usize;
        let y1 = self.y.max(0.0).min(image.height as f64 - 1.0) as usize;
        let x2 = (self.x + self.width)
            .max(0.0)
            .min(image.width as f64 - 1.0) as usize;
        let y2 = (self.y + self.height)
            .max(0.0)
            .min(image.height as f64 - 1.0) as usize;
        
        // Use a simple color conversion for now
        let color = self.fill_color.unwrap_or(crate::Color::BLACK);
        let pixel = Pixel::new(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        );
        
        // Draw the rectangle outline
        for x in x1..=x2 {
            image.set_pixel(x, y1, pixel);
            image.set_pixel(x, y2, pixel);
        }
        
        for y in y1..=y2 {
            image.set_pixel(x1, y, pixel);
            image.set_pixel(x2, y, pixel);
        }
    }
    
    fn rasterize_filled(&self, image: &mut PixelImage) {
        let x1 = self.x.max(0.0) as usize;
        let y1 = self.y.max(0.0) as usize;
        let x2 = (self.x + self.width).min(image.width as f64 - 1.0) as usize;
        let y2 = (self.y + self.height).min(image.height as f64 - 1.0) as usize;
        
        // Use a simple color conversion for now
        let color = self.fill_color.unwrap_or(crate::Color::BLACK);
        let pixel = Pixel::new(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        );
        
        // Fill the rectangle
        for y in y1..=y2 {
            for x in x1..=x2 {
                image.set_pixel(x, y, pixel);
            }
        }
        
        // If there's a stroke, draw the outline as well
        if let Some(stroke) = &self.stroke {
            let stroke_color = stroke.color;
            let stroke_pixel = Pixel::new(
                (stroke_color.r * 255.0) as u8,
                (stroke_color.g * 255.0) as u8,
                (stroke_color.b * 255.0) as u8,
                (stroke_color.a * 255.0) as u8,
            );
            
            // Draw the rectangle outline
            for x in x1..=x2 {
                image.set_pixel(x, y1, stroke_pixel);
                image.set_pixel(x, y2, stroke_pixel);
            }
            
            for y in y1..=y2 {
                image.set_pixel(x1, y, stroke_pixel);
                image.set_pixel(x2, y, stroke_pixel);
            }
        }
    }
}

impl Rasterizer for Circle {
    fn rasterize(&self, image: &mut PixelImage) {
        let cx = self.x as i32;
        let cy = self.y as i32;
        let r = self.radius as i32;
        
        // Use a simple color conversion for now
        let color = self.fill_color.unwrap_or(crate::Color::BLACK);
        let pixel = Pixel::new(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        );
        
        // Midpoint circle algorithm
        let mut x = r;
        let mut y = 0;
        let mut err = 0;
        
        while x >= y {
            // Draw points in all octants
            if cx + x >= 0 && cx + x < image.width as i32 && 
               cy + y >= 0 && cy + y < image.height as i32 {
                image.set_pixel((cx + x) as usize, (cy + y) as usize, pixel);
            }
            if cx + y >= 0 && cx + y < image.width as i32 && 
               cy + x >= 0 && cy + x < image.height as i32 {
                image.set_pixel((cx + y) as usize, (cy + x) as usize, pixel);
            }
            if cx - y >= 0 && cx - y < image.width as i32 && 
               cy + x >= 0 && cy + x < image.height as i32 {
                image.set_pixel((cx - y) as usize, (cy + x) as usize, pixel);
            }
            if cx - x >= 0 && cx - x < image.width as i32 && 
               cy + y >= 0 && cy + y < image.height as i32 {
                image.set_pixel((cx - x) as usize, (cy + y) as usize, pixel);
            }
            if cx - x >= 0 && cx - x < image.width as i32 && 
               cy - y >= 0 && cy - y < image.height as i32 {
                image.set_pixel((cx - x) as usize, (cy - y) as usize, pixel);
            }
            if cx - y >= 0 && cx - y < image.width as i32 && 
               cy - x >= 0 && cy - x < image.height as i32 {
                image.set_pixel((cx - y) as usize, (cy - x) as usize, pixel);
            }
            if cx + y >= 0 && cx + y < image.width as i32 && 
               cy - x >= 0 && cy - x < image.height as i32 {
                image.set_pixel((cx + y) as usize, (cy - x) as usize, pixel);
            }
            if cx + x >= 0 && cx + x < image.width as i32 && 
               cy - y >= 0 && cy - y < image.height as i32 {
                image.set_pixel((cx + x) as usize, (cy - y) as usize, pixel);
            }
            
            y += 1;
            err += 1 + 2 * y;
            if 2 * (err - x) + 1 > 0 {
                x -= 1;
                err += 1 - 2 * x;
            }
        }
    }
    
    fn rasterize_filled(&self, image: &mut PixelImage) {
        let cx = self.x as i32;
        let cy = self.y as i32;
        let r = self.radius as i32;
        
        // Use a simple color conversion for now
        let color = self.fill_color.unwrap_or(crate::Color::BLACK);
        let pixel = Pixel::new(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        );
        
        // Scanline fill algorithm for circles
        let r_squared = r * r;
        for y in -r..=r {
            let y_abs = y.abs();
            let x_limit = ((r_squared - y_abs * y_abs) as f32).sqrt() as i32;
            
            for x in -x_limit..=x_limit {
                let px = cx + x;
                let py = cy + y;
                
                if px >= 0 && px < image.width as i32 && 
                   py >= 0 && py < image.height as i32 {
                    image.set_pixel(px as usize, py as usize, pixel);
                }
            }
        }
        
        // If there's a stroke, draw the outline as well
        if let Some(stroke) = &self.stroke {
            let stroke_color = stroke.color;
            let stroke_pixel = Pixel::new(
                (stroke_color.r * 255.0) as u8,
                (stroke_color.g * 255.0) as u8,
                (stroke_color.b * 255.0) as u8,
                (stroke_color.a * 255.0) as u8,
            );
            
            // Midpoint circle algorithm for the outline
            let mut x = r;
            let mut y = 0;
            let mut err = 0;
            
            while x >= y {
                // Draw points in all octants
                if cx + x >= 0 && cx + x < image.width as i32 && 
                   cy + y >= 0 && cy + y < image.height as i32 {
                    image.set_pixel((cx + x) as usize, (cy + y) as usize, stroke_pixel);
                }
                if cx + y >= 0 && cx + y < image.width as i32 && 
                   cy + x >= 0 && cy + x < image.height as i32 {
                    image.set_pixel((cx + y) as usize, (cy + x) as usize, stroke_pixel);
                }
                if cx - y >= 0 && cx - y < image.width as i32 && 
                   cy + x >= 0 && cy + x < image.height as i32 {
                    image.set_pixel((cx - y) as usize, (cy + x) as usize, stroke_pixel);
                }
                if cx - x >= 0 && cx - x < image.width as i32 && 
                   cy + y >= 0 && cy + y < image.height as i32 {
                    image.set_pixel((cx - x) as usize, (cy + y) as usize, stroke_pixel);
                }
                if cx - x >= 0 && cx - x < image.width as i32 && 
                   cy - y >= 0 && cy - y < image.height as i32 {
                    image.set_pixel((cx - x) as usize, (cy - y) as usize, stroke_pixel);
                }
                if cx - y >= 0 && cx - y < image.width as i32 && 
                   cy - x >= 0 && cy - x < image.height as i32 {
                    image.set_pixel((cx - y) as usize, (cy - x) as usize, stroke_pixel);
                }
                if cx + y >= 0 && cx + y < image.width as i32 && 
                   cy - x >= 0 && cy - x < image.height as i32 {
                    image.set_pixel((cx + y) as usize, (cy - x) as usize, stroke_pixel);
                }
                if cx + x >= 0 && cx + x < image.width as i32 && 
                   cy - y >= 0 && cy - y < image.height as i32 {
                    image.set_pixel((cx + x) as usize, (cy - y) as usize, stroke_pixel);
                }
                
                y += 1;
                err += 1 + 2 * y;
                if 2 * (err - x) + 1 > 0 {
                    x -= 1;
                    err += 1 - 2 * x;
                }
            }
        }
    }
}

impl Rasterizer for Line {
    fn rasterize(&self, image: &mut PixelImage) {
        let x1 = self.x1 as i32;
        let y1 = self.y1 as i32;
        let x2 = self.x2 as i32;
        let y2 = self.y2 as i32;
        
        // Use a simple color conversion for now
        let stroke = self.stroke.unwrap_or_default();
        let color = stroke.color;
        let pixel = Pixel::new(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        );
        
        // Bresenham's line algorithm
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;
        
        let mut x = x1;
        let mut y = y1;
        
        loop {
            // Draw the pixel if it's within bounds
            if x >= 0 && x < image.width as i32 && 
               y >= 0 && y < image.height as i32 {
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
    
    fn rasterize_filled(&self, image: &mut PixelImage) {
        // Lines don't have a filled version, so we just call the regular rasterize
        self.rasterize(image);
    }
}

impl Rasterizer for Ellipse {
    fn rasterize(&self, image: &mut PixelImage) {
        let cx = self.x as i32;
        let cy = self.y as i32;
        let rx = self.radius_x as i32;
        let ry = self.radius_y as i32;
        
        // Use a simple color conversion for now
        let color = self.fill_color.unwrap_or(crate::Color::BLACK);
        let pixel = Pixel::new(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        );
        
        // Midpoint ellipse algorithm
        let mut x = 0;
        let mut y = ry;
        let rx2 = rx * rx;
        let ry2 = ry * ry;
        let crit1 = -(rx2 / 4 + rx % 2) + ry2;
        let crit2 = -(ry2 / 4 + ry % 2) + rx2;
        let _crit3 = -(ry2 / 4 + ry % 2) + rx2;
        let mut t = -rx2 * y;
        let mut dxt = 2 * ry2 * x;
        let mut dyt = -2 * rx2 * y;
        
        // Plot first set of points
        while y >= 0 && t <= crit1 {
            // Draw points in all quadrants
            plot_ellipse_points(image, cx, cy, x, y, pixel);
            
            x += 1;
            dxt += 2 * ry2;
            t += dxt;
        }
        
        // Plot second set of points
        while y >= 0 {
            plot_ellipse_points(image, cx, cy, x, y, pixel);
            
            y -= 1;
            dyt += 2 * rx2;
            t += dyt;
            
            if t > crit2 {
                x -= 1;
                dxt -= 2 * ry2;
                t += rx2 - dxt;
            }
        }
    }
    
    fn rasterize_filled(&self, image: &mut PixelImage) {
        let cx = self.x as i32;
        let cy = self.y as i32;
        let rx = self.radius_x as i32;
        let ry = self.radius_y as i32;
        
        // Use a simple color conversion for now
        let color = self.fill_color.unwrap_or(crate::Color::BLACK);
        let pixel = Pixel::new(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        );
        
        // Scanline fill algorithm for ellipses
        let rx_squared = rx * rx;
        let ry_squared = ry * ry;
        
        for y in -ry..=ry {
            let y_abs = y.abs();
            if ry_squared == 0 {
                continue;
            }
            let x_limit = ((rx_squared * (ry_squared - y_abs * y_abs)) as f32 / ry_squared as f32).sqrt() as i32;
            
            for x in -x_limit..=x_limit {
                let px = cx + x;
                let py = cy + y;
                
                if px >= 0 && px < image.width as i32 && 
                   py >= 0 && py < image.height as i32 {
                    image.set_pixel(px as usize, py as usize, pixel);
                }
            }
        }
        
        // If there's a stroke, draw the outline as well
        if let Some(stroke) = &self.stroke {
            let stroke_color = stroke.color;
            let stroke_pixel = Pixel::new(
                (stroke_color.r * 255.0) as u8,
                (stroke_color.g * 255.0) as u8,
                (stroke_color.b * 255.0) as u8,
                (stroke_color.a * 255.0) as u8,
            );
            
            // Midpoint ellipse algorithm for the outline
            let mut x = 0;
            let mut y = ry;
            let rx2 = rx * rx;
            let ry2 = ry * ry;
            let crit1 = -(rx2 / 4 + rx % 2) + ry2;
            let crit2 = -(ry2 / 4 + ry % 2) + rx2;
            let _crit3 = -(ry2 / 4 + ry % 2) + rx2;
            let mut t = -rx2 * y;
            let mut dxt = 2 * ry2 * x;
            let mut dyt = -2 * rx2 * y;
            
            // Plot first set of points
            while y >= 0 && t <= crit1 {
                // Draw points in all quadrants
                plot_ellipse_points(image, cx, cy, x, y, stroke_pixel);
                
                x += 1;
                dxt += 2 * ry2;
                t += dxt;
            }
            
            // Plot second set of points
            while y >= 0 {
                plot_ellipse_points(image, cx, cy, x, y, stroke_pixel);
                
                y -= 1;
                dyt += 2 * rx2;
                t += dyt;
                
                if t > crit2 {
                    x -= 1;
                    dxt -= 2 * ry2;
                    t += rx2 - dxt;
                }
            }
        }
    }
}

/// Helper function to plot ellipse points in all quadrants
fn plot_ellipse_points(image: &mut PixelImage, cx: i32, cy: i32, x: i32, y: i32, pixel: Pixel) {
    // Quadrant 1
    if cx + x >= 0 && cx + x < image.width as i32 && 
       cy + y >= 0 && cy + y < image.height as i32 {
        image.set_pixel((cx + x) as usize, (cy + y) as usize, pixel);
    }
    
    // Quadrant 2
    if cx - x >= 0 && cx - x < image.width as i32 && 
       cy + y >= 0 && cy + y < image.height as i32 {
        image.set_pixel((cx - x) as usize, (cy + y) as usize, pixel);
    }
    
    // Quadrant 3
    if cx - x >= 0 && cx - x < image.width as i32 && 
       cy - y >= 0 && cy - y < image.height as i32 {
        image.set_pixel((cx - x) as usize, (cy - y) as usize, pixel);
    }
    
    // Quadrant 4
    if cx + x >= 0 && cx + x < image.width as i32 && 
       cy - y >= 0 && cy - y < image.height as i32 {
        image.set_pixel((cx + x) as usize, (cy - y) as usize, pixel);
    }
}

impl Rasterizer for Polygon {
    fn rasterize(&self, image: &mut PixelImage) {
        if self.points.is_empty() {
            return;
        }
        
        // Use a simple color conversion for now
        let color = self.fill_color.unwrap_or(crate::Color::BLACK);
        let _pixel = Pixel::new(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        );
        
        // Draw lines between consecutive points
        for i in 0..self.points.len() {
            let start = self.points[i];
            let end = self.points[(i + 1) % self.points.len()];
            
            // Create a temporary line and rasterize it
            let line = Line {
                x1: start.0,
                y1: start.1,
                x2: end.0,
                y2: end.1,
                stroke: self.stroke,
            };
            
            line.rasterize(image);
        }
    }
    
    fn rasterize_filled(&self, image: &mut PixelImage) {
        if self.points.is_empty() {
            return;
        }
        
        // Use a simple color conversion for now
        let color = self.fill_color.unwrap_or(crate::Color::BLACK);
        let pixel = Pixel::new(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        );
        
        // Find the bounding box of the polygon
        let mut min_x = self.points[0].0;
        let mut max_x = self.points[0].0;
        let mut min_y = self.points[0].1;
        let mut max_y = self.points[0].1;
        
        for point in &self.points {
            min_x = min_x.min(point.0);
            max_x = max_x.max(point.0);
            min_y = min_y.min(point.1);
            max_y = max_y.max(point.1);
        }
        
        // Clamp to image bounds
        let min_x = min_x.max(0.0) as usize;
        let max_x = max_x.min(image.width as f64 - 1.0) as usize;
        let min_y = min_y.max(0.0) as usize;
        let max_y = max_y.min(image.height as f64 - 1.0) as usize;
        
        // Scanline fill algorithm
        for y in min_y..=max_y {
            let mut intersections = Vec::new();
            
            // Find intersections with polygon edges
            for i in 0..self.points.len() {
                let p1 = self.points[i];
                let p2 = self.points[(i + 1) % self.points.len()];
                
                // Check if the edge crosses the current scanline
                if (p1.1 > y as f64) != (p2.1 > y as f64) {
                    // Calculate the intersection point
                    let x = p1.0 + (y as f64 - p1.1) * (p2.0 - p1.0) / (p2.1 - p1.1);
                    intersections.push(x);
                }
            }
            
            // Sort intersections
            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            // Fill between pairs of intersections
            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    let x1 = intersections[i].max(min_x as f64).max(0.0) as usize;
                    let x2 = intersections[i + 1].min(max_x as f64).min(image.width as f64 - 1.0) as usize;
                    
                    for x in x1..=x2 {
                        image.set_pixel(x, y, pixel);
                    }
                }
            }
        }
        
        // If there's a stroke, draw the outline as well
        if let Some(stroke) = &self.stroke {
            let stroke_color = stroke.color;
            let stroke_pixel = Pixel::new(
                (stroke_color.r * 255.0) as u8,
                (stroke_color.g * 255.0) as u8,
                (stroke_color.b * 255.0) as u8,
                (stroke_color.a * 255.0) as u8,
            );
            
            // Draw lines between consecutive points
            for i in 0..self.points.len() {
                let start = self.points[i];
                let end = self.points[(i + 1) % self.points.len()];
                
                // Create a temporary line and rasterize it
                let line = Line {
                    x1: start.0,
                    y1: start.1,
                    x2: end.0,
                    y2: end.1,
                    stroke: Some(*stroke),
                };
                
                line.rasterize(image);
            }
            
            // Use stroke_pixel to avoid unused variable warning
            let _ = stroke_pixel;
        }
    }
}
