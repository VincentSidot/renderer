//! Example demonstrating bezier curve support

use renderer::{
    backend::{PNGBackend, SVGBackend},
    builder::ShapeBuilder,
    color::Color,
    image::Image,
    shape::{BezierCurveBuilder},
    stroke,
};

#[cfg(feature = "svg")]
fn render_svg() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = Image::new().with_width(500.0).with_height(300.0);

    // Create a quadratic bezier curve
    let quadratic_bezier = BezierCurveBuilder::new()
        .add_point(50.0, 200.0)  // Start point
        .add_point(150.0, 50.0)  // Control point
        .add_point(250.0, 200.0) // End point
        .with_stroke(
            stroke::Stroke::new()
                .with_size(2.0)
                .with_color(Color::rgb8(0xFF, 0x00, 0x00)), // Red
        )
        .build()?;

    image.add(quadratic_bezier);

    // Create a cubic bezier curve
    let cubic_bezier = BezierCurveBuilder::new()
        .add_point(300.0, 100.0)  // Start point
        .add_point(350.0, 50.0)   // First control point
        .add_point(400.0, 150.0)  // Second control point
        .add_point(450.0, 100.0)  // End point
        .with_stroke(
            stroke::Stroke::new()
                .with_size(2.0)
                .with_color(Color::rgb8(0x00, 0xFF, 0x00)), // Green
        )
        .build()?;

    image.add(cubic_bezier);

    // Create a more complex bezier curve with multiple segments
    let complex_bezier = BezierCurveBuilder::new()
        .add_point(50.0, 50.0)   // Start point
        .add_point(100.0, 100.0) // Control point 1
        .add_point(150.0, 0.0)   // Control point 2
        .add_point(200.0, 50.0)  // Control point 3
        .add_point(250.0, 100.0) // Control point 4
        .add_point(300.0, 0.0)   // End point
        .with_stroke(
            stroke::Stroke::new()
                .with_size(2.0)
                .with_color(Color::rgb8(0x00, 0x00, 0xFF)), // Blue
        )
        .build()?;

    image.add(complex_bezier);

    let mut svg = SVGBackend::new();
    let path = std::path::Path::new("./trash/bezier_output.svg");
    image.save(path, &mut svg)?;

    println!("Bezier curve SVG file created at: {}", path.display());

    Ok(())
}

#[cfg(feature = "png")]
fn render_png() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = Image::new().with_width(500.0).with_height(300.0);

    // Create a quadratic bezier curve
    let quadratic_bezier = BezierCurveBuilder::new()
        .add_point(50.0, 200.0)  // Start point
        .add_point(150.0, 50.0)  // Control point
        .add_point(250.0, 200.0) // End point
        .with_stroke(
            stroke::Stroke::new()
                .with_size(2.0)
                .with_color(Color::rgb8(0xFF, 0x00, 0x00)), // Red
        )
        .build()?;

    image.add(quadratic_bezier);

    // Create a cubic bezier curve
    let cubic_bezier = BezierCurveBuilder::new()
        .add_point(300.0, 100.0)  // Start point
        .add_point(350.0, 50.0)   // First control point
        .add_point(400.0, 150.0)  // Second control point
        .add_point(450.0, 100.0)  // End point
        .with_stroke(
            stroke::Stroke::new()
                .with_size(2.0)
                .with_color(Color::rgb8(0x00, 0xFF, 0x00)), // Green
        )
        .build()?;

    image.add(cubic_bezier);

    // Create a more complex bezier curve with multiple segments
    let complex_bezier = BezierCurveBuilder::new()
        .add_point(50.0, 50.0)   // Start point
        .add_point(100.0, 100.0) // Control point 1
        .add_point(150.0, 0.0)   // Control point 2
        .add_point(200.0, 50.0)  // Control point 3
        .add_point(250.0, 100.0) // Control point 4
        .add_point(300.0, 0.0)   // End point
        .with_stroke(
            stroke::Stroke::new()
                .with_size(2.0)
                .with_color(Color::rgb8(0x00, 0x00, 0xFF)), // Blue
        )
        .build()?;

    image.add(complex_bezier);

    let mut png = PNGBackend::new();
    let path = std::path::Path::new("./trash/bezier_output.png");
    image.save(path, &mut png)?;

    println!("Bezier curve PNG file created at: {}", path.display());

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "svg")]
    render_svg()?;

    #[cfg(feature = "png")]
    render_png()?;

    Ok(())
}
