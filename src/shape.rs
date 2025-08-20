//! Shape module for the renderer

use crate::{color::Color, stroke::Stroke};

/// A rectangle shape
#[derive(Debug, Clone)]
pub struct Rectangle {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
    /// Width
    pub width: f32,
    /// Height
    pub height: f32,
    /// Fill color
    pub fill_color: Option<Color>,
    /// Stroke
    pub stroke: Option<Stroke>,
}

impl Rectangle {
    /// Create a new rectangle
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            fill_color: None,
            stroke: None,
        }
    }

    /// Set the position of the rectangle
    pub fn with_pos(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set the size of the rectangle
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set the fill color of the rectangle
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set the stroke of the rectangle
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new()
    }
}

/// A circle shape
#[derive(Debug, Clone)]
pub struct Circle {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
    /// Radius
    pub radius: f32,
    /// Fill color
    pub fill_color: Option<Color>,
    /// Stroke
    pub stroke: Option<Stroke>,
}

impl Circle {
    /// Create a new circle
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            radius: 0.0,
            fill_color: None,
            stroke: None,
        }
    }

    /// Set the position of the circle
    pub fn with_pos(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set the radius of the circle
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Set the fill color of the circle
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set the stroke of the circle
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self::new()
    }
}

/// A line shape
#[derive(Debug, Clone)]
pub struct Line {
    /// Start X position
    pub x1: f32,
    /// Start Y position
    pub y1: f32,
    /// End X position
    pub x2: f32,
    /// End Y position
    pub y2: f32,
    /// Stroke
    pub stroke: Option<Stroke>,
}

impl Line {
    /// Create a new line
    pub fn new() -> Self {
        Self {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
            stroke: None,
        }
    }

    /// Set the start position of the line
    pub fn with_pos(mut self, x: f32, y: f32) -> Self {
        self.x1 = x;
        self.y1 = y;
        self
    }

    /// Set the end position of the line
    pub fn with_end(mut self, x: f32, y: f32) -> Self {
        self.x2 = x;
        self.y2 = y;
        self
    }

    /// Set the stroke of the line
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

/// A text shape
#[derive(Debug, Clone)]
pub struct Text {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
    /// Text content
    pub content: String,
    /// Font size
    pub font_size: f32,
    /// Fill color
    pub fill_color: Option<Color>,
}

impl Text {
    /// Create a new text
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            content: String::new(),
            font_size: 16.0,
            fill_color: None,
        }
    }

    /// Set the position of the text
    pub fn with_pos(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set the text content
    pub fn with_text<S: Into<String>>(mut self, text: S) -> Self {
        self.content = text.into();
        self
    }

    /// Set the font size
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set the fill color of the text
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}

/// An ellipse shape
#[derive(Debug, Clone)]
pub struct Ellipse {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
    /// Radius along the x-axis
    pub radius_x: f32,
    /// Radius along the y-axis
    pub radius_y: f32,
    /// Fill color
    pub fill_color: Option<Color>,
    /// Stroke
    pub stroke: Option<Stroke>,
}

impl Ellipse {
    /// Create a new ellipse
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            radius_x: 0.0,
            radius_y: 0.0,
            fill_color: None,
            stroke: None,
        }
    }

    /// Set the position of the ellipse
    pub fn with_pos(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set the radii of the ellipse
    pub fn with_radii(mut self, radius_x: f32, radius_y: f32) -> Self {
        self.radius_x = radius_x;
        self.radius_y = radius_y;
        self
    }

    /// Set the fill color of the ellipse
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set the stroke of the ellipse
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

impl Default for Ellipse {
    fn default() -> Self {
        Self::new()
    }
}

/// A polygon shape
#[derive(Debug, Clone)]
pub struct Polygon {
    /// Points of the polygon (x, y)
    pub points: Vec<(f32, f32)>,
    /// Fill color
    pub fill_color: Option<Color>,
    /// Stroke
    pub stroke: Option<Stroke>,
}

impl Polygon {
    /// Create a new polygon
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            fill_color: None,
            stroke: None,
        }
    }

    /// Add a point to the polygon
    pub fn add_point(mut self, x: f32, y: f32) -> Self {
        self.points.push((x, y));
        self
    }

    /// Set multiple points for the polygon
    pub fn with_points(mut self, points: Vec<(f32, f32)>) -> Self {
        self.points = points;
        self
    }

    /// Set the fill color of the polygon
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set the stroke of the polygon
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

impl Default for Polygon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ellipse_builder() {
        let ellipse = Ellipse::new()
            .with_pos(10.0, 20.0)
            .with_radii(30.0, 40.0)
            .with_fill_color(Color::RED)
            .with_stroke(Stroke::new().with_size(2.0).with_color(Color::BLUE));

        assert_eq!(ellipse.x, 10.0);
        assert_eq!(ellipse.y, 20.0);
        assert_eq!(ellipse.radius_x, 30.0);
        assert_eq!(ellipse.radius_y, 40.0);
        assert_eq!(ellipse.fill_color, Some(Color::RED));
        assert_eq!(ellipse.stroke.unwrap().width, 2.0);
        assert_eq!(ellipse.stroke.unwrap().color, Color::BLUE);
    }

    #[test]
    fn test_polygon_builder() {
        let polygon = Polygon::new()
            .add_point(0.0, 0.0)
            .add_point(10.0, 0.0)
            .add_point(10.0, 10.0)
            .add_point(0.0, 10.0)
            .with_fill_color(Color::GREEN)
            .with_stroke(Stroke::new().with_size(1.0).with_color(Color::BLACK));

        assert_eq!(polygon.points.len(), 4);
        assert_eq!(polygon.points[0], (0.0, 0.0));
        assert_eq!(polygon.points[1], (10.0, 0.0));
        assert_eq!(polygon.points[2], (10.0, 10.0));
        assert_eq!(polygon.points[3], (0.0, 10.0));
        assert_eq!(polygon.fill_color, Some(Color::GREEN));
        assert_eq!(polygon.stroke.unwrap().width, 1.0);
        assert_eq!(polygon.stroke.unwrap().color, Color::BLACK);
    }

    #[test]
    fn test_polygon_with_points() {
        let points = vec![(0.0, 0.0), (5.0, 5.0), (10.0, 0.0)];
        let polygon = Polygon::new().with_points(points.clone());

        assert_eq!(polygon.points, points);
    }
}

/// Enum representing all possible shapes
#[derive(Debug, Clone)]
pub enum Shape {
    /// Rectangle shape
    Rectangle(Rectangle),
    /// Circle shape
    Circle(Circle),
    /// Line shape
    Line(Line),
    /// Text shape
    Text(Text),
    /// Ellipse shape
    Ellipse(Ellipse),
    /// Polygon shape
    Polygon(Polygon),
}
