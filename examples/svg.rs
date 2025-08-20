// renderer imports goes here

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = Image::new().with_width(800).with_height(600.0);

    image.add(
        shape::Rectangle::new()
            .with_pos(20.0, 40)
            .with_size(50, 50)
            .with_fill_color(color::RED)
            .with_stroke(
                stroke::new()
                    .with_size(1.0)
                    .with_color(color::rgb8(0x20, 0x20, 0x20)),
            ),
    );

    image.add(
        shape::Circle::new()
            .with_pos(100.0, 100.0)
            .with_radius(50.0)
            .with_fill_color(color::rgba(1.0, 0.0, 0.0, 0.5)),
    );

    image.add(
        shape::Line::new()
            .with_pos(200.0, 200.0)
            .with_end(300.0, 300.0)
            .with_stroke(
                stroke::new()
                    .with_size(2.0)
                    .with_color(color::rgb8(0x00, 0x00, 0xFF)),
            ),
    );

    image.add(
        shape::Text::new()
            .with_pos(400.0, 400.0)
            .with_text("Hello, SVG!")
            .with_font_size(24.0)
            .with_fill_color(color::rgb(0.0, 0.0, 1.0)),
    );

    let mut svg = backend::SVGBackend::init();
    let path = std::path::Path::new("output.svg");
    image.save(path, &mut svg)?;

    println!("SVG file created at: {}", path.display());

    Ok(())
}
