//! Example demonstrating rasterization

#[cfg(feature = "rasterizer")]
use renderer::{
    PixelImage,
    builder::ShapeBuilder,
    color::Color,
    rasterizer::Rasterizer,
    shape::{CircleBuilder, LineBuilder, PolygonBuilder, RectangleBuilder},
    stroke,
};

#[cfg(feature = "rasterizer")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a pixel image
    let mut image = PixelImage::new(200, 200);

    // Clear the image with white
    image.fill(renderer::Pixel::WHITE);

    // Create and rasterize a rectangle
    let rectangle = RectangleBuilder::new()
        .with_pos(10.0, 10.0)
        .with_size(50.0, 30.0)
        .with_fill_color(Color::RED)
        .build()?;

    rectangle.rasterize_filled(&mut image);

    // Create and rasterize a circle
    let circle = CircleBuilder::new()
        .with_center(100.0, 100.0)
        .with_radius(25.0)
        .with_fill_color(Color::BLUE)
        .build()?;

    circle.rasterize_filled(&mut image);

    // Create and rasterize a line
    let line = LineBuilder::new()
        .with_start(0.0, 0.0)
        .with_end(200.0, 200.0)
        .with_stroke(
            stroke::Stroke::new()
                .with_size(1.0)
                .with_color(Color::GREEN),
        )
        .build()?;

    line.rasterize(&mut image); // Lines don't have a filled version

    // Create and rasterize a polygon (triangle)
    let triangle = PolygonBuilder::new()
        .add_point(150.0, 50.0)
        .add_point(180.0, 80.0)
        .add_point(120.0, 80.0)
        .with_fill_color(Color::GREEN)
        .build()?;

    triangle.rasterize_filled(&mut image);

    // Save as a simple PPM image
    save_as_ppm(&image, "./trash/output.ppm")?;

    println!("Rasterized image saved to ./trash/output.ppm");

    Ok(())
}

#[cfg(not(feature = "rasterizer"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rasterizer feature is not enabled");
    Ok(())
}

/// Save the pixel image as a PPM image
#[cfg(feature = "rasterizer")]
fn save_as_ppm(image: &PixelImage, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(path)?;

    // Write PPM header
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", image.width, image.height)?;
    writeln!(file, "255")?;

    // Write pixel data
    for pixel in &image.pixels {
        writeln!(file, "{} {} {}", pixel.r, pixel.g, pixel.b)?;
    }

    Ok(())
}
