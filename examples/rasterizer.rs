//! Example demonstrating rasterization

use renderer::{
    color::Color,
    rasterizer::{RasterBuffer, Rasterizer},
    shape::{Circle, Line, Polygon, Rectangle},
    stroke,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a raster buffer
    let mut buffer = RasterBuffer::new(200, 200);
    
    // Clear the buffer with white
    buffer.fill(renderer::rasterizer::Pixel::WHITE);
    
    // Create and rasterize a rectangle
    let rectangle = Rectangle::new()
        .with_pos(10.0, 10.0)
        .with_size(50.0, 30.0)
        .with_fill_color(Color::RED);
    
    rectangle.rasterize(&mut buffer);
    
    // Create and rasterize a circle
    let circle = Circle::new()
        .with_pos(100.0, 100.0)
        .with_radius(25.0)
        .with_fill_color(Color::BLUE);
    
    circle.rasterize(&mut buffer);
    
    // Create and rasterize a line
    let line = Line::new()
        .with_pos(0.0, 0.0)
        .with_end(200.0, 200.0)
        .with_stroke(
            stroke::Stroke::new()
                .with_size(1.0)
                .with_color(Color::GREEN),
        );
    
    line.rasterize(&mut buffer);
    
    // Create and rasterize a polygon (triangle)
    let triangle = Polygon::new()
        .add_point(150.0, 50.0)
        .add_point(180.0, 80.0)
        .add_point(120.0, 80.0)
        .with_fill_color(Color::GREEN);
    
    triangle.rasterize(&mut buffer);
    
    // Save as a simple PPM image
    save_as_ppm(&buffer, "./trash/output.ppm")?;
    
    println!("Rasterized image saved to ./trash/output.ppm");
    
    Ok(())
}

/// Save the raster buffer as a PPM image
fn save_as_ppm(buffer: &RasterBuffer, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Write;
    
    let mut file = File::create(path)?;
    
    // Write PPM header
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", buffer.width, buffer.height)?;
    writeln!(file, "255")?;
    
    // Write pixel data
    for pixel in &buffer.pixels {
        writeln!(file, "{} {} {}", pixel.r, pixel.g, pixel.b)?;
    }
    
    Ok(())
}