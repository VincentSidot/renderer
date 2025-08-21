//! Example demonstrating font rasterization

#[cfg(feature = "rasterizer")]
use renderer::rasterizer::{FontRenderer, PixelImage};

#[cfg(feature = "rasterizer")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a pixel image
    let mut image = PixelImage::new(400, 200);

    // Clear the image with white
    image.fill(renderer::rasterizer::Pixel::WHITE);

    // Create a font renderer with the default font
    match FontRenderer::default(24.0) {
        Ok(font) => {
            // Rasterize some text
            let color = renderer::rasterizer::Pixel::rgb(0, 0, 255); // Blue text
            font.rasterize_text(&mut image, "Hello, World!\nWelcome Back", 50, 100, color)?;

            // Save as a simple PPM image
            save_as_ppm(&image, "./trash/font_output.ppm")?;
            println!("Font rasterization example saved to ./trash/font_output.ppm");
        }
        Err(e) => {
            println!("Font rasterization not available: {}", e);
        }
    }

    // Example of loading a font from bytes
    #[cfg(feature = "rasterizer")]
    {
        // This would normally load actual font data
        // let font_data = include_bytes!("path/to/font.ttf");
        // match FontRenderer::new(font_data.as_slice(), 24.0) {
        //     Ok(font) => {
        //         // Use the font...
        //     }
        //     Err(e) => {
        //         println!("Failed to load font from bytes: {}", e);
        //     }
        // }
    }

    // Example of loading a font from a file path
    #[cfg(feature = "rasterizer")]
    {
        // This would normally load a font from a file
        // match FontRenderer::new("./font/ubuntu.ttf", 24.0) {
        //     Ok(font) => {
        //         // Use the font...
        //     }
        //     Err(e) => {
        //         println!("Failed to load font from file: {}", e);
        //     }
        // }
    }

    Ok(())
}

#[cfg(not(feature = "rasterizer"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rasterizer feature is not enabled");
    Ok(())
}

/// Save the pixel image as a PPM image
#[cfg(feature = "rasterizer")]
fn save_as_ppm(image: &PixelImage, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(path)?;

    // Write PPM header
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", image.width, image.height)?;
    writeln!(file, "255")?;

    // Write pixel data
    for pixel in &image.pixels {
        writeln!(file, "{} {} {}", pixel.r, pixel.g, pixel.b)?;
    }

    Ok(())
}
