//! SVG backend implementation

use crate::{image::Image, shape::Shape};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// SVG backend for rendering images
#[derive(Debug)]
pub struct SVGBackend;

impl SVGBackend {
    /// Initialize a new SVG backend
    pub fn init() -> Self {
        Self
    }
}

impl super::Backend for SVGBackend {
    fn render(&mut self, image: &Image, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(path)?;

        // Write SVG header
        writeln!(
            file,
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            image.width, image.height
        )?;

        // Render each shape
        for shape in &image.shapes {
            match shape {
                Shape::Rectangle(rect) => {
                    write!(
                        file,
                        r#"  <rect x="{}" y="{}" width="{}" height="{}""#,
                        rect.x, rect.y, rect.width, rect.height
                    )?;

                    if let Some(color) = rect.fill_color {
                        write!(file, r#" fill="{color}""#)?;
                    }

                    if let Some(stroke) = rect.stroke {
                        write!(
                            file,
                            r#" stroke="{}" stroke-width="{}""#,
                            stroke.color, stroke.width
                        )?;
                    }

                    writeln!(file, "/>")?;
                }
                Shape::Circle(circle) => {
                    write!(
                        file,
                        r#"  <circle cx="{}" cy="{}" r="{}""#,
                        circle.x, circle.y, circle.radius
                    )?;

                    if let Some(color) = circle.fill_color {
                        write!(file, r#" fill="{color}""#)?;
                    }

                    if let Some(stroke) = circle.stroke {
                        write!(
                            file,
                            r#" stroke="{}" stroke-width="{}""#,
                            stroke.color, stroke.width
                        )?;
                    }

                    writeln!(file, "/>")?;
                }
                Shape::Line(line) => {
                    write!(
                        file,
                        r#"  <line x1="{}" y1="{}" x2="{}" y2="{}""#,
                        line.x1, line.y1, line.x2, line.y2
                    )?;

                    if let Some(stroke) = line.stroke {
                        write!(
                            file,
                            r#" stroke="{}" stroke-width="{}""#,
                            stroke.color, stroke.width
                        )?;
                    }

                    writeln!(file, "/>")?;
                }
                Shape::Text(text) => {
                    write!(
                        file,
                        r#"  <text x="{}" y="{}" font-size="{}""#,
                        text.x, text.y, text.font_size
                    )?;

                    if let Some(color) = text.fill_color {
                        write!(file, r#" fill="{color}""#)?;
                    }

                    // Escape special characters in text content
                    let escaped_content = escape_text_content(&text.content);
                    writeln!(file, ">{escaped_content}</text>")?;
                }
                Shape::Ellipse(ellipse) => {
                    write!(
                        file,
                        r#"  <ellipse cx="{}" cy="{}" rx="{}" ry="{}""#,
                        ellipse.x, ellipse.y, ellipse.radius_x, ellipse.radius_y
                    )?;

                    if let Some(color) = ellipse.fill_color {
                        write!(file, r#" fill="{color}""#)?;
                    }

                    if let Some(stroke) = ellipse.stroke {
                        write!(
                            file,
                            r#" stroke="{}" stroke-width="{}""#,
                            stroke.color, stroke.width
                        )?;
                    }

                    writeln!(file, "/>")?;
                }
                Shape::Polygon(polygon) => {
                    // Format points as "x1,y1 x2,y2 x3,y3 ..."
                    let points_str: String = polygon
                        .points
                        .iter()
                        .map(|(x, y)| format!("{x},{y}"))
                        .collect::<Vec<_>>()
                        .join(" ");

                    write!(file, r#"  <polygon points="{points_str}""#)?;

                    if let Some(color) = polygon.fill_color {
                        write!(file, r#" fill="{color}""#)?;
                    }

                    if let Some(stroke) = polygon.stroke {
                        write!(
                            file,
                            r#" stroke="{}" stroke-width="{}""#,
                            stroke.color, stroke.width
                        )?;
                    }

                    writeln!(file, "/>")?;
                }
            }
        }

        // Write SVG footer
        writeln!(file, "</svg>")?;

        Ok(())
    }
}

/// Escapes special characters in SVG text content
fn escape_text_content(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_escape_text_content() {
        // Test basic escaping
        assert_eq!(escape_text_content("&"), "&amp;");
        assert_eq!(escape_text_content("<"), "&lt;");
        assert_eq!(escape_text_content(">"), "&gt;");
        assert_eq!(escape_text_content("\""), "&quot;");
        assert_eq!(escape_text_content("'"), "&apos;");

        // Test combined escaping
        assert_eq!(
            escape_text_content("Hello & welcome <to> \"SVG\""),
            "Hello &amp; welcome &lt;to&gt; &quot;SVG&quot;"
        );
    }

    #[test]
    fn test_svg_text_escaping() {
        let mut image = Image::new();
        let mut backend = SVGBackend::init();

        // Add text with special characters
        image.add(Shape::Text(
            crate::shape::Text::new()
                .with_pos(10.0, 10.0)
                .with_text("Hello & welcome <to> \"SVG\"")
                .with_font_size(12.0),
        ));

        // Create a temporary file path
        let path = Path::new("test_output.svg");

        // Render the image
        assert!(image.save(path, &mut backend).is_ok());

        // Clean up
        let _ = std::fs::remove_file(path);
    }
}
