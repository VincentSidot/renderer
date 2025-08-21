mod circle;
mod ellipse;
mod line;
mod polygon;
mod rect;
mod tests;
mod text;

pub use circle::Circle;
pub use ellipse::Ellipse;
pub use line::Line;
pub use polygon::Polygon;
pub use rect::Rectangle;
pub use text::Text;

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
