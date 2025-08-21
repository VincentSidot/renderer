//! Example demonstrating SVG rendering

use renderer::{backend::SVGBackend, color::Color, image::Image, shape::*, stroke};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = Image::new().with_width(800.0).with_height(600.0);

    image.add(
        Rectangle::new()
            .with_pos(20.0, 40.0)
            .with_size(50.0, 50.0)
            .with_fill_color(Color::RED)
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(1.0)
                    .with_color(Color::rgb8(0x20, 0x20, 0x20)),
            ),
    );

    image.add(
        Circle::new()
            .with_pos(100.0, 100.0)
            .with_radius(50.0)
            .with_fill_color(Color::rgba(1.0, 0.0, 0.0, 0.5)),
    );

    image.add(
        Line::new()
            .with_pos(200.0, 200.0)
            .with_end(300.0, 300.0)
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(2.0)
                    .with_color(Color::rgb8(0x00, 0x00, 0xFF)),
            ),
    );

    image.add(
        Text::new()
            .with_pos(400.0, 400.0)
            .with_text("Hello, SVG!")
            .with_font_size(24.0)
            .with_fill_color(Color::rgb(0.0, 0.0, 1.0)),
    );

    // Add new shapes
    image.add(
        Ellipse::new()
            .with_pos(600.0, 100.0)
            .with_radii(60.0, 40.0)
            .with_fill_color(Color::GREEN)
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(2.0)
                    .with_color(Color::BLACK),
            ),
    );

    image.add(
        Polygon::new()
            .add_point(500.0, 300.0)
            .add_point(550.0, 250.0)
            .add_point(600.0, 300.0)
            .add_point(575.0, 350.0)
            .add_point(525.0, 350.0)
            .with_fill_color(Color::BLUE)
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(1.0)
                    .with_color(Color::BLACK),
            ),
    );

    let mut svg = SVGBackend::init();
    let path = std::path::Path::new("./trash/output.svg");
    image.save(path, &mut svg)?;

    println!("SVG file created at: {}", path.display());

    Ok(())
}
