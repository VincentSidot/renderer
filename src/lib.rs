//! Renderer crate with optional logging capabilities and SVG rendering

#[cfg(feature = "logger")]
mod logger;

#[cfg(feature = "logger")]
pub use logger::init_logger;

// Re-export log::Level to make it accessible from outside the module
pub use log::Level;
