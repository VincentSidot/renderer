mod builded;
mod builder;

pub use builded::circle::Circle;
pub use builded::ellipse::Ellipse;
pub use builded::line::Line;
pub use builded::polygon::Polygon;
pub use builded::rect::Rectangle;
pub use builded::text::Text;

pub use builder::circle::CircleBuilder;
pub use builder::ellipse::EllipseBuilder;
pub use builder::line::LineBuilder;
pub use builder::polygon::PolygonBuilder;
pub use builder::rect::RectangleBuilder;
pub use builder::text::TextBuilder;

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
