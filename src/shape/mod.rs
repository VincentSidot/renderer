mod circle;
mod circle_builder;
mod ellipse;
mod ellipse_builder;
mod line;
mod line_builder;
mod polygon;
mod polygon_builder;
mod rect;
mod rect_builder;
mod tests;
mod text;
mod text_builder;

pub use circle::Circle;
pub use circle_builder::CircleBuilder;
pub use ellipse::Ellipse;
pub use ellipse_builder::EllipseBuilder;
pub use line::Line;
pub use line_builder::LineBuilder;
pub use polygon::Polygon;
pub use polygon_builder::PolygonBuilder;
pub use rect::Rectangle;
pub use rect_builder::RectangleBuilder;
pub use text::Text;
pub use text_builder::TextBuilder;

// /// Enum representing all possible shapes
// #[derive(Debug, Clone)]
// pub enum Shape {
//     /// Rectangle shape
//     Rectangle(Rectangle),
//     /// Circle shape
//     Circle(Circle),
//     /// Line shape
//     Line(Line),
//     /// Text shape
//     Text(Text),
//     /// Ellipse shape
//     Ellipse(Ellipse),
//     /// Polygon shape
//     Polygon(Polygon),
// }

macro_rules! impl_shape {
    (
        $(
            $(#[doc = $doc:expr])*
            $name:ident
        ),*
        $(,)? // Trailing comma for convenience
    ) => {


        /// Enum representing all possible shapes
        #[derive(Debug, Clone)]
        pub enum Shape {
            $(
                $(#[doc = $doc])?
                $name($name),
            )*
        }

        $(
            impl From<$name> for Shape {
                fn from(shape: $name) -> Shape {
                    Shape::$name(shape)
                }
            }
        )*
    };
}

impl_shape! {
    /// Rectangle shape
    Rectangle,
    /// Circle shape
    Circle,
    /// Line shape
    Line,
    /// Text shape
    Text,
    /// Ellipse shape
    Ellipse,
    /// Polygon shape
    Polygon,
}
