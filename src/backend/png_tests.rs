#[cfg(test)]
#[cfg(feature = "png")]
mod tests {
    use super::*;
    use crate::backend::PNGBackend;
    use crate::image::Image;
    use crate::shape::{self, Shape};
    use crate::color::Color;
    use std::path::Path;

    #[test]
    fn test_png_backend_creation() {
        let backend = PNGBackend::new();
        assert!(true); // Just checking that creation works
    }

    #[test]
    fn test_png_backend_with_font_data() {
        let font_data = vec![0u8; 100]; // Dummy font data
        let backend = PNGBackend::with_font_data(font_data);
        assert!(true); // Just checking that creation works
    }
}