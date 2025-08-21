mod circle;
mod ellipse;
mod line;
mod polygon;
mod rect;
mod text;

pub use circle::Circle;
pub use ellipse::Ellipse;
pub use line::Line;
pub use polygon::Polygon;
pub use rect::Rectangle;
pub use text::Text;

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