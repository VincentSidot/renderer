use renderer::{
    builder::{BuildError, ShapeBuilder},
    shape::{
        CircleBuilder, EllipseBuilder, LineBuilder, PolygonBuilder, RectangleBuilder, TextBuilder,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test RectangleBuilder
    let rectangle = RectangleBuilder::new()
        .with_pos(10.0, 20.0)
        .with_size(50.0, 30.0)
        .build()?;
    println!("Rectangle: {:?}", rectangle);

    // Test CircleBuilder
    let circle = CircleBuilder::new()
        .with_center(50.0, 50.0)
        .with_radius(25.0)
        .build()?;
    println!("Circle: {:?}", circle);

    // Test CircleBuilder with diameter
    let circle2 = CircleBuilder::new()
        .with_center(100.0, 100.0)
        .with_diameter(50.0)
        .build()?;
    println!("Circle2: {:?}", circle2);

    // Test CircleBuilder from area
    let area = 25.0 * std::f64::consts::PI; // Area of circle with radius 5
    let circle3 = CircleBuilder::new()
        .with_center(150.0, 150.0)
        .from_area(area)
        .build()?;
    println!("Circle3: {:?}", circle3);

    // Test EllipseBuilder
    let ellipse = EllipseBuilder::new()
        .with_center(200.0, 200.0)
        .with_radii(30.0, 20.0)
        .build()?;
    println!("Ellipse: {:?}", ellipse);

    // Test LineBuilder
    let line = LineBuilder::new()
        .with_start(0.0, 0.0)
        .with_end(100.0, 100.0)
        .build()?;
    println!("Line: {:?}", line);

    // Test TextBuilder
    let text = TextBuilder::new()
        .with_pos(10.0, 20.0)
        .with_text("Hello, World!")
        .with_font_size(16.0)
        .build()?;
    println!("Text: {:?}", text);

    // Test PolygonBuilder
    let polygon = PolygonBuilder::new()
        .add_point(0.0, 0.0)
        .add_point(10.0, 0.0)
        .add_point(10.0, 10.0)
        .add_point(0.0, 10.0)
        .build()?;
    println!("Polygon: {:?}", polygon);

    // Test error cases
    let result = RectangleBuilder::new().with_size(-1.0, 30.0).build();
    assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    println!("Correctly caught invalid rectangle width");

    let result = CircleBuilder::new().with_radius(5.0).with_diameter(10.0).build();
    assert!(matches!(result, Err(BuildError::ConflictingProperties(_))));
    println!("Correctly caught conflicting circle properties");

    println!("All builder tests passed!");
    Ok(())
}