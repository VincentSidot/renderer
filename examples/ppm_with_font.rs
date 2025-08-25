//! Example demonstrating PPM rendering with custom font

#[cfg(feature = "ppm")]
use renderer::{
    backend::PPMBackend,
    builder::ShapeBuilder,
    color::Color,
    image::Image,
    shape::{CircleBuilder, RectangleBuilder, Shape, TextBuilder},
    stroke,
};

#[cfg(feature = "ppm")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = Image::new().with_width(400.0).with_height(300.0);

    // Add some shapes
    image.add(Shape::Rectangle(
        RectangleBuilder::new()
            .with_pos(20.0, 40.0)
            .with_size(50.0, 50.0)
            .with_fill_color(Color::RED)
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(1.0)
                    .with_color(Color::rgb8(0x20, 0x20, 0x20)),
            )
            .build()?,
    ));

    image.add(Shape::Circle(
        CircleBuilder::new()
            .with_center(100.0, 100.0)
            .with_radius(50.0)
            .with_fill_color(Color::rgba(1.0, 0.0, 0.0, 0.5))
            .build()?,
    ));

    // Add text with the default font
    image.add(Shape::Text(
        TextBuilder::new()
            .with_pos(200.0, 250.0)
            .with_text("Hello, PPM with Font!")
            .with_font_size(24.0)
            .with_fill_color(Color::rgb8(0x00, 0x00, 0x00))
            .build()?,
    ));

    // Create a PPM backend with the default font
    let mut ppm = PPMBackend::new();
    let path = std::path::Path::new("./trash/output_with_default_font.ppm");
    image.save(path, &mut ppm)?;

    println!(
        "Binary PPM file with default font created at: {}",
        path.display()
    );

    // Create a PPM backend with custom font data (in this case, we'll use the same Ubuntu font)
    #[cfg(feature = "rasterizer")]
    {
        let font_data = include_bytes!("../font/ubuntu.ttf").to_vec();
        let mut ppm_with_font = PPMBackend::with_font_data(font_data);
        let path = std::path::Path::new("./trash/output_with_custom_font.ppm");
        image.save(path, &mut ppm_with_font)?;

        println!(
            "Binary PPM file with custom font created at: {}",
            path.display()
        );
    }

    Ok(())
}

#[cfg(not(feature = "ppm"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("PPM feature is not enabled");
    Ok(())
}
