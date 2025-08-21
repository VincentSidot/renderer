//! Example demonstrating ASCII PPM rendering

#[cfg(feature = "ppm")]
use renderer::{
    backend::AsciiPPMBackend,
    color::Color,
    image::Image,
    shape::{self, Shape},
    stroke,
};

#[cfg(feature = "ppm")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = Image::new().with_width(200.0).with_height(200.0);

    image.add(Shape::Rectangle(
        shape::Rectangle::new()
            .with_pos(20.0, 40.0)
            .with_size(50.0, 50.0)
            .with_fill_color(Color::RED)
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(1.0)
                    .with_color(Color::rgb8(0x20, 0x20, 0x20)),
            ),
    ));

    image.add(Shape::Circle(
        shape::Circle::new()
            .with_pos(100.0, 100.0)
            .with_radius(50.0)
            .with_fill_color(Color::rgba(1.0, 0.0, 0.0, 0.5)),
    ));

    image.add(Shape::Line(
        shape::Line::new()
            .with_pos(0.0, 0.0)
            .with_end(200.0, 200.0)
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(2.0)
                    .with_color(Color::rgb8(0x00, 0x00, 0xFF)),
            ),
    ));

    image.add(Shape::Ellipse(
        shape::Ellipse::new()
            .with_pos(150.0, 50.0)
            .with_radii(30.0, 20.0)
            .with_fill_color(Color::GREEN)
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(2.0)
                    .with_color(Color::BLACK),
            ),
    ));

    image.add(Shape::Polygon(
        shape::Polygon::new()
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
            ),
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