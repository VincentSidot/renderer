//! Test demonstrating the updated shape builders with different numeric types

#[cfg(test)]
mod tests {
    use crate::shape::{Circle, Ellipse, Line, Polygon, Rectangle, Text};

    #[test]
    fn test_rectangle_with_different_numeric_types() {
        // Test with integers
        let rect1 = Rectangle::new()
            .with_pos(10i32, 20i32)
            .with_size(50i32, 30i32);
        assert_eq!(rect1.x, 10.0);
        assert_eq!(rect1.y, 20.0);
        assert_eq!(rect1.width, 50.0);
        assert_eq!(rect1.height, 30.0);

        // Test with floats
        let rect2 = Rectangle::new()
            .with_pos(10.5f64, 20.7f64)
            .with_size(50.5f64, 30.7f64);
        assert_eq!(rect2.x, 10.5);
        assert_eq!(rect2.y, 20.7);
        assert_eq!(rect2.width, 50.5);
        assert_eq!(rect2.height, 30.7);

        // Test with mixed types
        let rect3 = Rectangle::new()
            .with_pos(10u32, 20.7f64)
            .with_size(50.5f64, 30i16);
        assert_eq!(rect3.x, 10.0);
        assert_eq!(rect3.y, 20.7);
        assert_eq!(rect3.width, 50.5);
        assert_eq!(rect3.height, 30.0);
    }

    #[test]
    fn test_circle_with_different_numeric_types() {
        // Test with integers
        let circle1 = Circle::new().with_pos(50i32, 50i32).with_radius(25i32);
        assert_eq!(circle1.x, 50.0);
        assert_eq!(circle1.y, 50.0);
        assert_eq!(circle1.radius, 25.0);

        // Test with floats
        let circle2 = Circle::new()
            .with_pos(50.5f64, 50.7f64)
            .with_radius(25.5f64);
        assert_eq!(circle2.x, 50.5);
        assert_eq!(circle2.y, 50.7);
        assert_eq!(circle2.radius, 25.5);

        // Test with mixed types
        let circle3 = Circle::new().with_pos(50u32, 50.7f64).with_radius(25i16);
        assert_eq!(circle3.x, 50.0);
        assert_eq!(circle3.y, 50.7);
        assert_eq!(circle3.radius, 25.0);
    }

    #[test]
    fn test_line_with_different_numeric_types() {
        // Test with integers
        let line1 = Line::new().with_pos(0i32, 0i32).with_end(100i32, 100i32);
        assert_eq!(line1.x1, 0.0);
        assert_eq!(line1.y1, 0.0);
        assert_eq!(line1.x2, 100.0);
        assert_eq!(line1.y2, 100.0);

        // Test with floats
        let line2 = Line::new()
            .with_pos(0.5f64, 0.7f64)
            .with_end(100.5f64, 100.7f64);
        assert_eq!(line2.x1, 0.5);
        assert_eq!(line2.y1, 0.7);
        assert_eq!(line2.x2, 100.5);
        assert_eq!(line2.y2, 100.7);

        // Test with mixed types
        let line3 = Line::new().with_pos(0u32, 0.7f64).with_end(100i16, 100u32);
        assert_eq!(line3.x1, 0.0);
        assert_eq!(line3.y1, 0.7);
        assert_eq!(line3.x2, 100.0);
        assert_eq!(line3.y2, 100.0);
    }

    #[test]
    fn test_text_with_different_numeric_types() {
        // Test with integers
        let text1 = Text::new().with_pos(10i32, 20i32).with_font_size(16i32);
        assert_eq!(text1.x, 10.0);
        assert_eq!(text1.y, 20.0);
        assert_eq!(text1.font_size, 16.0);

        // Test with floats
        let text2 = Text::new()
            .with_pos(10.5f64, 20.7f64)
            .with_font_size(16.5f64);
        assert_eq!(text2.x, 10.5);
        assert_eq!(text2.y, 20.7);
        assert_eq!(text2.font_size, 16.5);

        // Test with mixed types
        let text3 = Text::new().with_pos(10u32, 20.7f64).with_font_size(16i16);
        assert_eq!(text3.x, 10.0);
        assert_eq!(text3.y, 20.7);
        assert_eq!(text3.font_size, 16.0);
    }

    #[test]
    fn test_ellipse_with_different_numeric_types() {
        // Test with integers
        let ellipse1 = Ellipse::new()
            .with_pos(50i32, 50i32)
            .with_radii(30i32, 20i32);
        assert_eq!(ellipse1.x, 50.0);
        assert_eq!(ellipse1.y, 50.0);
        assert_eq!(ellipse1.radius_x, 30.0);
        assert_eq!(ellipse1.radius_y, 20.0);

        // Test with floats
        let ellipse2 = Ellipse::new()
            .with_pos(50.5f64, 50.7f64)
            .with_radii(30.5f64, 20.7f64);
        assert_eq!(ellipse2.x, 50.5);
        assert_eq!(ellipse2.y, 50.7);
        assert_eq!(ellipse2.radius_x, 30.5);
        assert_eq!(ellipse2.radius_y, 20.7);

        // Test with mixed types
        let ellipse3 = Ellipse::new()
            .with_pos(50u32, 50.7f64)
            .with_radii(30i16, 20u32);
        assert_eq!(ellipse3.x, 50.0);
        assert_eq!(ellipse3.y, 50.7);
        assert_eq!(ellipse3.radius_x, 30.0);
        assert_eq!(ellipse3.radius_y, 20.0);
    }

    #[test]
    fn test_polygon_with_different_numeric_types() {
        // Test with integers
        let polygon1 = Polygon::new()
            .add_point(0i32, 0i32)
            .add_point(10i32, 0i32)
            .add_point(10i32, 10i32)
            .add_point(0i32, 10i32);
        assert_eq!(polygon1.points[0], (0.0, 0.0));
        assert_eq!(polygon1.points[1], (10.0, 0.0));
        assert_eq!(polygon1.points[2], (10.0, 10.0));
        assert_eq!(polygon1.points[3], (0.0, 10.0));

        // Test with floats
        let polygon2 = Polygon::new()
            .add_point(0.5f64, 0.7f64)
            .add_point(10.5f64, 0.7f64)
            .add_point(10.5f64, 10.7f64)
            .add_point(0.5f64, 10.7f64);
        assert_eq!(polygon2.points[0], (0.5, 0.7));
        assert_eq!(polygon2.points[1], (10.5, 0.7));
        assert_eq!(polygon2.points[2], (10.5, 10.7));
        assert_eq!(polygon2.points[3], (0.5, 10.7));

        // Test with mixed types
        let polygon3 = Polygon::new()
            .add_point(0u32, 0.7f64)
            .add_point(10i16, 0.7f64)
            .add_point(10i16, 10u32)
            .add_point(0u32, 10i16);
        assert_eq!(polygon3.points[0], (0.0, 0.7));
        assert_eq!(polygon3.points[1], (10.0, 0.7));
        assert_eq!(polygon3.points[2], (10.0, 10.0));
        assert_eq!(polygon3.points[3], (0.0, 10.0));
    }
}
