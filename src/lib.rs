//! Renderer crate with optional logging capabilities and SVG rendering

#[cfg(feature = "logger")]
mod logger;

#[cfg(feature = "logger")]
pub use logger::init_logger;

// Re-export log::Level to make it accessible from outside the module
pub use log::Level;

#[cfg(feature = "svg")]
pub mod backend;
#[cfg(feature = "svg")]
pub mod color;
#[cfg(feature = "svg")]
pub mod image;
#[cfg(feature = "svg")]
pub mod shape;
#[cfg(feature = "svg")]
pub mod stroke;

#[cfg(feature = "svg")]
pub use backend::*;
#[cfg(feature = "svg")]
pub use color::*;
#[cfg(feature = "svg")]
pub use image::*;
#[cfg(feature = "svg")]
pub use shape::*;
#[cfg(feature = "svg")]
pub use stroke::*;
