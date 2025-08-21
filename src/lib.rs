//! Renderer crate with optional logging capabilities and SVG rendering

#[cfg(feature = "logger")]
mod logger;

#[cfg(feature = "logger")]
pub use logger::init_logger;

// Re-export log::Level to make it accessible from outside the module
pub use log::Level;

pub mod backend;
pub mod builder;
pub mod color;
pub mod image;
#[cfg(feature = "rasterizer")]
pub mod rasterizer;
pub mod shape;
pub mod stroke;

pub use builder::{BuildError, ShapeBuilder};
pub use color::Color;
pub use image::Image;
#[cfg(feature = "rasterizer")]
pub use rasterizer::{Pixel, PixelImage};
pub use shape::{Circle, Ellipse, Line, Polygon, Rectangle, Shape, Text};
pub use stroke::Stroke;
