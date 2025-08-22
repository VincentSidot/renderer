//! Tests for the Shape enum's TryInto implementations

#[cfg(test)]
mod tests {
    use crate::shape::*;
    use crate::builder::*;
    use std::convert::TryInto;

    #[test]
    fn test_shape_try_into_rectangle() {
        // Create a rectangle using the builder
        let rectangle = RectangleBuilder::new()
            .with_pos(10.0, 20.0)
            .with_size(50.0, 30.0)
            .build()
            .expect("Failed to build rectangle");
        
        // Convert to Shape
        let shape: Shape = rectangle.into();
        
        // Try to convert back to Rectangle
        let converted_rectangle: Result<Rectangle, _> = shape.try_into();
        assert!(converted_rectangle.is_ok());
        
        let rect = converted_rectangle.unwrap();
        assert_eq!(rect.x(), 10.0);
        assert_eq!(rect.y(), 20.0);
        assert_eq!(rect.width(), 50.0);
        assert_eq!(rect.height(), 30.0);
    }

    #[test]
    fn test_shape_try_into_circle() {
        // Create a circle using the builder
        let circle = CircleBuilder::new()
            .with_center(50.0, 50.0)
            .with_radius(25.0)
            .build()
            .expect("Failed to build circle");
        
        // Convert to Shape
        let shape: Shape = circle.into();
        
        // Try to convert back to Circle
        let converted_circle: Result<Circle, _> = shape.try_into();
        assert!(converted_circle.is_ok());
        
        let circ = converted_circle.unwrap();
        assert_eq!(circ.x(), 50.0);
        assert_eq!(circ.y(), 50.0);
        assert_eq!(circ.radius(), 25.0);
    }

    #[test]
    fn test_shape_try_into_wrong_type() {
        // Create a rectangle using the builder
        let rectangle = RectangleBuilder::new()
            .with_pos(10.0, 20.0)
            .with_size(50.0, 30.0)
            .build()
            .expect("Failed to build rectangle");
        
        // Convert to Shape
        let shape: Shape = rectangle.into();
        
        // Try to convert to Circle (should fail)
        let converted_circle: Result<Circle, _> = shape.try_into();
        assert!(converted_circle.is_err());
    }
}