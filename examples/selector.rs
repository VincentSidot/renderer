use std::path::PathBuf;

use renderer::{
    Color, Image, ShapeBuilder,
    backend::{PNGBackend, SVGBackend},
    shape::{
        CircleBuilder, EllipseBuilder, LineBuilder, PolygonBuilder, RectangleBuilder, TextBuilder,
    },
    stroke,
};

fn draw(image: &mut Image) -> Result<(), Box<dyn std::error::Error>> {
    let width = image.width();
    let height = image.height();

    // --- Background ---------------------------------------------------------
    image.add(
        RectangleBuilder::new()
            .with_pos(0.0, 0.0)
            .with_size(width, height)
            .with_fill_color(Color::rgb8(0x12, 0x14, 0x1C)) // near-black blue
            .build()?,
    );

    // Subtle vignette / panel shadow (fake “shadow” by offset, low alpha)
    image.add(
        RectangleBuilder::new()
            .with_pos(90.0, 100.0)
            .with_size(1020.0, 520.0)
            .with_fill_color(Color::rgba(0.0, 0.0, 0.0, 0.25))
            .build()?,
    );

    // --- Main panel ---------------------------------------------------------
    image.add(
        RectangleBuilder::new()
            .with_pos(80.0, 90.0)
            .with_size(1020.0, 520.0)
            .with_fill_color(Color::rgb8(0x1F, 0x23, 0x35)) // dark slate
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(2.0)
                    .with_color(Color::rgb8(0x36, 0x3C, 0x55)),
            )
            .build()?,
    );

    // Header bar
    image.add(
        RectangleBuilder::new()
            .with_pos(80.0, 90.0)
            .with_size(1020.0, 68.0)
            .with_fill_color(Color::rgb8(0x2C, 0x34, 0x55)) // indigo-ish
            .build()?,
    );

    // Header title
    image.add(
        TextBuilder::new()
            .with_pos(110.0, 135.0)
            .with_text("Styled Drawing — Demo")
            .with_font_size(28.0)
            .with_fill_color(Color::rgb8(0xE5, 0xEC, 0xFF))
            .build()?,
    );

    // --- Accent circle with ring & glow ------------------------------------
    // Soft glow (bigger, translucent)
    image.add(
        CircleBuilder::new()
            .with_center(260.0, 300.0)
            .with_radius(110.0)
            .with_fill_color(Color::rgba(0.1, 0.6, 1.0, 0.12))
            .build()?,
    );

    // Main accent circle
    image.add(
        CircleBuilder::new()
            .with_center(260.0, 300.0)
            .with_radius(90.0)
            .with_fill_color(Color::rgb8(0x3B, 0xC3, 0xFF)) // cyan
            .build()?,
    );

    // Inner ring (stroke-only)
    image.add(
        CircleBuilder::new()
            .with_center(260.0, 300.0)
            .with_radius(70.0)
            .with_fill_color(Color::rgba(0.0, 0.0, 0.0, 0.0))
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(6.0)
                    .with_color(Color::rgb8(0x12, 0x14, 0x1C)),
            )
            .build()?,
    );

    // Highlight dot
    image.add(
        CircleBuilder::new()
            .with_center(230.0, 275.0)
            .with_radius(14.0)
            .with_fill_color(Color::rgba(1.0, 1.0, 1.0, 0.7))
            .build()?,
    );

    // --- Info block ---------------------------------------------------------
    // Card shadow
    image.add(
        RectangleBuilder::new()
            .with_pos(420.0, 210.0)
            .with_size(600.0, 320.0)
            .with_fill_color(Color::rgba(0.0, 0.0, 0.0, 0.25))
            .build()?,
    );

    // Card
    image.add(
        RectangleBuilder::new()
            .with_pos(410.0, 200.0)
            .with_size(600.0, 320.0)
            .with_fill_color(Color::rgb8(0x23, 0x28, 0x3D))
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(1.5)
                    .with_color(Color::rgb8(0x40, 0x48, 0x6B)),
            )
            .build()?,
    );

    // Subtitle
    image.add(
        TextBuilder::new()
            .with_pos(440.0, 240.0)
            .with_text("Card Title")
            .with_font_size(22.0)
            .with_fill_color(Color::rgb8(0xD0, 0xDA, 0xFF))
            .build()?,
    );

    // Separator line
    image.add(
        LineBuilder::new()
            .with_start(440.0, 255.0)
            .with_end(980.0, 255.0)
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(1.0)
                    .with_color(Color::rgb8(0x40, 0x48, 0x6B)),
            )
            .build()?,
    );

    // Body text (3 lines)
    for (i, text) in [
        "This is a styled block demonstrating:",
        "• layering (fake shadows via translucent duplicates)",
        "• strokes, fills, and semi-transparent accents",
    ]
    .iter()
    .enumerate()
    {
        image.add(
            TextBuilder::new()
                .with_pos(440.0, 290.0 + (i as f32) * 28.0)
                .with_text(text)
                .with_font_size(18.0)
                .with_fill_color(Color::rgb8(0xA8, 0xB2, 0xD8))
                .build()?,
        );
    }

    // --- Decorative polygon ribbon -----------------------------------------
    image.add(
        PolygonBuilder::new()
            .add_point(410.0, 360.0)
            .add_point(760.0, 360.0)
            .add_point(800.0, 400.0)
            .add_point(760.0, 440.0)
            .add_point(410.0, 440.0)
            .with_fill_color(Color::rgb8(0x59, 0x66, 0x99))
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(1.0)
                    .with_color(Color::rgb8(0x36, 0x3C, 0x55)),
            )
            .build()?,
    );

    // Ribbon label
    image.add(
        TextBuilder::new()
            .with_pos(440.0, 408.0)
            .with_text("Ribbon Accent")
            .with_font_size(20.0)
            .with_fill_color(Color::rgb8(0xF2, 0xF6, 0xFF))
            .build()?,
    );

    // --- Subtle decorative ellipse -----------------------------------------
    image.add(
        EllipseBuilder::new()
            .with_center(970.0, 470.0)
            .with_radii(70.0, 22.0)
            .with_fill_color(Color::rgba(0.95, 0.6, 0.1, 0.25))
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(1.0)
                    .with_color(Color::rgb8(0x8A, 0x52, 0x1A)),
            )
            .build()?,
    );

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <output_file>", args[0]);
        std::process::exit(1);
    }
    let filename = PathBuf::from(&args[1]);

    let width = 1200.0;
    let height = 720.0;

    let mut image = Image::new().with_size(width, height);

    // Draw time

    draw(&mut image)?;

    // Save time
    let mut backend = match filename.extension().and_then(|x| x.to_str()) {
        Some("png") => Box::new(PNGBackend::new()) as Box<dyn renderer::backend::Backend>,
        Some("svg") => Box::new(SVGBackend::new()) as Box<dyn renderer::backend::Backend>,
        _ => {
            return Err(format!("Unsupported file extension: {:?}", filename.extension()).into());
        }
    };

    image.save(filename, backend.as_mut())?;

    Ok(())
}
