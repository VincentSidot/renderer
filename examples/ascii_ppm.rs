//! Example demonstrating ASCII PPM rendering

#[cfg(feature = "ppm")]
use renderer::{
    backend::AsciiPPMBackend,
    builder::ShapeBuilder,
    color::Color,
    image::Image,
    shape::{
        CircleBuilder, EllipseBuilder, LineBuilder, PolygonBuilder, RectangleBuilder, Shape,
        TextBuilder,
    },
    stroke,
};

#[cfg(feature = "ppm")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = Image::new().with_width(400.0).with_height(300.0);

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

    image.add(Shape::Line(
        LineBuilder::new()
            .with_start(0.0, 0.0)
            .with_end(200.0, 200.0)
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(2.0)
                    .with_color(Color::rgb8(0x00, 0x00, 0xFF)),
            )
            .build()?,
    ));

    image.add(Shape::Ellipse(
        EllipseBuilder::new()
            .with_center(150.0, 50.0)
            .with_radii(30.0, 20.0)
            .with_fill_color(Color::GREEN)
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(2.0)
                    .with_color(Color::BLACK),
            )
            .build()?,
    ));

    image.add(Shape::Polygon(
        PolygonBuilder::new()
            .add_point(50.0, 150.0)
            .add_point(100.0, 100.0)
            .add_point(150.0, 150.0)
            .add_point(125.0, 175.0)
            .add_point(75.0, 175.0)
            .with_fill_color(Color::BLUE)
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(1.0)
                    .with_color(Color::BLACK),
            )
            .build()?,
    ));

    // Add text with different font sizes
    image.add(Shape::Text(
        TextBuilder::new()
            .with_pos(200.0, 250.0)
            .with_text("Small text")
            .with_font_size(12.0)
            .with_fill_color(Color::rgb8(0x00, 0x00, 0x00))
            .build()?,
    ));

    image.add(Shape::Text(
        TextBuilder::new()
            .with_pos(200.0, 270.0)
            .with_text("Medium text")
            .with_font_size(24.0)
            .with_fill_color(Color::rgb8(0x00, 0x00, 0x00))
            .build()?,
    ));

    image.add(Shape::Text(
        TextBuilder::new()
            .with_pos(200.0, 300.0)
            .with_text("Large text")
            .with_font_size(36.0)
            .with_fill_color(Color::rgb8(0x00, 0x00, 0x00))
            .build()?,
    ));

    let mut ppm = AsciiPPMBackend::new();
    let path = std::path::Path::new("./trash/output_ascii.ppm");
    image.save(path, &mut ppm)?;

    println!("ASCII PPM file created at: {}", path.display());

    Ok(())
}

#[cfg(not(feature = "ppm"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("PPM feature is not enabled");
    Ok(())
}
