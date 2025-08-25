mod builded;
mod builder;

pub use builded::bezier::BezierCurve;
pub use builded::circle::Circle;
pub use builded::ellipse::Ellipse;
pub use builded::line::Line;
pub use builded::polygon::Polygon;
pub use builded::rect::Rectangle;
pub use builded::text::Text;

pub use builder::bezier::BezierCurveBuilder;
pub use builder::circle::CircleBuilder;
pub use builder::ellipse::EllipseBuilder;
pub use builder::line::LineBuilder;
pub use builder::polygon::PolygonBuilder;
pub use builder::rect::RectangleBuilder;
pub use builder::text::TextBuilder;

use crate::builder::BuildError;

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

        $(
            impl TryFrom<Shape> for $name {
                type Error = BuildError;

                fn try_from(shape: Shape) -> Result<Self, Self::Error> {
                    match shape {
                        Shape::$name(inner) => Ok(inner),
                        _ => Err(BuildError::InvalidValue(format!("Expected {}, found {:?}", stringify!($name), shape))),
                    }
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
    /// Bezier curve shape
    BezierCurve,
}

#[cfg(test)]
mod tests;
