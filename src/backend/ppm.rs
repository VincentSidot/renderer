//! PPM Backend

#[derive(Debug)]
pub struct PPMBackend;

impl PPMBackend {}

impl super::Backend for PPMBackend {
    fn render(
        &mut self,
        _image: &crate::Image,
        _path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
