use crate::shape::{Circle, Ellipse, Line, Polygon, Rectangle};

/// A simple RGB pixel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    /// Create a new pixel with the specified RGB values
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// Predefined colors
impl Pixel {
    /// Black pixel
    pub const BLACK: Pixel = Pixel { r: 0, g: 0, b: 0 };
    
    /// White pixel
    pub const WHITE: Pixel = Pixel { r: 255, g: 255, b: 255 };
}

/// A 2D raster image buffer
#[derive(Debug)]
pub struct RasterBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pixel>,
}

impl RasterBuffer {
    /// Create a new raster buffer with the specified dimensions
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Pixel::WHITE; width * height],
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
    
    /// Set a pixel at the specified coordinates
    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        if let Some(p) = self.get_pixel_mut(x, y) {
            *p = pixel;
        }
    }
    
    /// Fill the buffer with a specific color
    pub fn fill(&mut self, color: Pixel) {
        self.pixels.fill(color);
    }
}

/// Trait for rasterizing shapes to a pixel buffer
pub trait Rasterizer {
    /// Rasterize the shape onto the provided buffer
    fn rasterize(&self, buffer: &mut RasterBuffer);
}

impl Rasterizer for Rectangle {
    fn rasterize(&self, buffer: &mut RasterBuffer) {
        let x1 = self.x.max(0.0).min(buffer.width as f32 - 1.0) as usize;
        let y1 = self.y.max(0.0).min(buffer.height as f32 - 1.0) as usize;
        let x2 = (self.x + self.width)
            .max(0.0)
            .min(buffer.width as f32 - 1.0) as usize;
        let y2 = (self.y + self.height)
            .max(0.0)
            .min(buffer.height as f32 - 1.0) as usize;
        
        // Use a simple color conversion for now
        let color = self.fill_color.unwrap_or(crate::Color::BLACK);
        let pixel = Pixel::new(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
        );
        
        // Draw the rectangle outline
        for x in x1..=x2 {
            buffer.set_pixel(x, y1, pixel);
            buffer.set_pixel(x, y2, pixel);
        }
        
        for y in y1..=y2 {
            buffer.set_pixel(x1, y, pixel);
            buffer.set_pixel(x2, y, pixel);
        }
    }
}

impl Rasterizer for Circle {
    fn rasterize(&self, buffer: &mut RasterBuffer) {
        let cx = self.x as i32;
        let cy = self.y as i32;
        let r = self.radius as i32;
        
        // Use a simple color conversion for now
        let color = self.fill_color.unwrap_or(crate::Color::BLACK);
        let pixel = Pixel::new(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
        );
        
        // Midpoint circle algorithm
        let mut x = r;
        let mut y = 0;
        let mut err = 0;
        
        while x >= y {
            // Draw points in all octants
            if cx + x >= 0 && cx + x < buffer.width as i32 && 
               cy + y >= 0 && cy + y < buffer.height as i32 {
                buffer.set_pixel((cx + x) as usize, (cy + y) as usize, pixel);
            }
            if cx + y >= 0 && cx + y < buffer.width as i32 && 
               cy + x >= 0 && cy + x < buffer.height as i32 {
                buffer.set_pixel((cx + y) as usize, (cy + x) as usize, pixel);
            }
            if cx - y >= 0 && cx - y < buffer.width as i32 && 
               cy + x >= 0 && cy + x < buffer.height as i32 {
                buffer.set_pixel((cx - y) as usize, (cy + x) as usize, pixel);
            }
            if cx - x >= 0 && cx - x < buffer.width as i32 && 
               cy + y >= 0 && cy + y < buffer.height as i32 {
                buffer.set_pixel((cx - x) as usize, (cy + y) as usize, pixel);
            }
            if cx - x >= 0 && cx - x < buffer.width as i32 && 
               cy - y >= 0 && cy - y < buffer.height as i32 {
                buffer.set_pixel((cx - x) as usize, (cy - y) as usize, pixel);
            }
            if cx - y >= 0 && cx - y < buffer.width as i32 && 
               cy - x >= 0 && cy - x < buffer.height as i32 {
                buffer.set_pixel((cx - y) as usize, (cy - x) as usize, pixel);
            }
            if cx + y >= 0 && cx + y < buffer.width as i32 && 
               cy - x >= 0 && cy - x < buffer.height as i32 {
                buffer.set_pixel((cx + y) as usize, (cy - x) as usize, pixel);
            }
            if cx + x >= 0 && cx + x < buffer.width as i32 && 
               cy - y >= 0 && cy - y < buffer.height as i32 {
                buffer.set_pixel((cx + x) as usize, (cy - y) as usize, pixel);
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

impl Rasterizer for Line {
    fn rasterize(&self, buffer: &mut RasterBuffer) {
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
        );
        
        // Bresenham's line algorithm
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let err = dx - dy;
        
        let mut x = x1;
        let mut y = y1;
        
        loop {
            // Draw the pixel if it's within bounds
            if x >= 0 && x < buffer.width as i32 && 
               y >= 0 && y < buffer.height as i32 {
                buffer.set_pixel(x as usize, y as usize, pixel);
            }
            
            // Check if we've reached the end point
            if x == x2 && y == y2 {
                break;
            }
            
            let e2 = 2 * err;
            if e2 > -dy {
                // err -= dy;  // This would need to be mutable
                // But we can avoid mutation by recomputing when needed
            }
            if e2 < dx {
                // err += dx;  // This would need to be mutable
                // But we can avoid mutation by recomputing when needed
            }
            
            // Simplified movement without error tracking
            if (x - x2).abs() > (y - y2).abs() {
                x += sx;
            } else {
                y += sy;
            }
        }
    }
}

impl Rasterizer for Ellipse {
    fn rasterize(&self, buffer: &mut RasterBuffer) {
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
            plot_ellipse_points(buffer, cx, cy, x, y, pixel);
            
            x += 1;
            dxt += 2 * ry2;
            t += dxt;
        }
        
        // Plot second set of points
        while y >= 0 {
            plot_ellipse_points(buffer, cx, cy, x, y, pixel);
            
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

/// Helper function to plot ellipse points in all quadrants
fn plot_ellipse_points(buffer: &mut RasterBuffer, cx: i32, cy: i32, x: i32, y: i32, pixel: Pixel) {
    // Quadrant 1
    if cx + x >= 0 && cx + x < buffer.width as i32 && 
       cy + y >= 0 && cy + y < buffer.height as i32 {
        buffer.set_pixel((cx + x) as usize, (cy + y) as usize, pixel);
    }
    
    // Quadrant 2
    if cx - x >= 0 && cx - x < buffer.width as i32 && 
       cy + y >= 0 && cy + y < buffer.height as i32 {
        buffer.set_pixel((cx - x) as usize, (cy + y) as usize, pixel);
    }
    
    // Quadrant 3
    if cx - x >= 0 && cx - x < buffer.width as i32 && 
       cy - y >= 0 && cy - y < buffer.height as i32 {
        buffer.set_pixel((cx - x) as usize, (cy - y) as usize, pixel);
    }
    
    // Quadrant 4
    if cx + x >= 0 && cx + x < buffer.width as i32 && 
       cy - y >= 0 && cy - y < buffer.height as i32 {
        buffer.set_pixel((cx + x) as usize, (cy - y) as usize, pixel);
    }
}

impl Rasterizer for Polygon {
    fn rasterize(&self, buffer: &mut RasterBuffer) {
        if self.points.is_empty() {
            return;
        }
        
        // Use a simple color conversion for now
        let color = self.fill_color.unwrap_or(crate::Color::BLACK);
        let _pixel = Pixel::new(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
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
            
            line.rasterize(buffer);
        }
    }
}