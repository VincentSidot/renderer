//! Backend module for the renderer

#[cfg(feature = "svg")]
mod svg;
#[cfg(feature = "svg")]
pub use svg::SVGBackend;

#[cfg(feature = "ppm")]
mod ascii_ppm;
#[cfg(feature = "ppm")]
pub use ascii_ppm::AsciiPPMBackend;

#[cfg(feature = "ppm")]
mod ppm;
#[cfg(feature = "ppm")]
pub use ppm::PPMBackend;

#[cfg(feature = "png")]
mod png;
#[cfg(feature = "png")]
pub use png::PNGBackend;

use crate::image::Image;
use std::path::Path;

/// Backend trait for rendering images
pub trait Backend {
    /// Render an image to a file
    fn render(&mut self, image: &Image, path: &Path) -> Result<(), Box<dyn std::error::Error>>;
}
