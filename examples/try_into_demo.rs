//! Example demonstrating the TryInto functionality for Shape enum

use renderer::{
    builder::{BuildError, ShapeBuilder},
    shape::{CircleBuilder, RectangleBuilder, Shape},
};
use std::convert::TryInto;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Shape TryInto Example");
    println!("====================");

    // Create a rectangle using the builder
    let rectangle = RectangleBuilder::new()
        .with_pos(10.0, 20.0)
        .with_size(50.0, 30.0)
        .build()?;

    println!("Created rectangle: {:?}", rectangle);

    // Convert to Shape
    let shape: Shape = rectangle.into();
    println!("Converted to shape: {:?}", shape);

    // Try to convert back to Rectangle
    let converted_rectangle: Result<renderer::shape::Rectangle, BuildError> = shape.try_into();
    match converted_rectangle {
        Ok(rect) => {
            println!("Successfully converted back to rectangle: {:?}", rect);
            println!(
                "Rectangle properties - x: {}, y: {}, width: {}, height: {}",
                rect.x(),
                rect.y(),
                rect.width(),
                rect.height()
            );
        }
        Err(e) => println!("Failed to convert back to rectangle: {}", e),
    }

    // Create a circle and try to convert it to a rectangle (should fail)
    let circle = CircleBuilder::new()
        .with_center(50.0, 50.0)
        .with_radius(25.0)
        .build()?;

    let circle_shape: Shape = circle.into();

    // Try to convert circle shape to rectangle (should fail)
    let converted_to_rectangle: Result<renderer::shape::Rectangle, BuildError> =
        circle_shape.try_into();
    match converted_to_rectangle {
        Ok(rect) => {
            println!("Unexpectedly converted circle to rectangle: {:?}", rect);
        }
        Err(e) => println!("Failed to convert circle to rectangle (as expected): {}", e),
    }

    println!("Example completed successfully!");
    Ok(())
}
