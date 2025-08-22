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
    pub fn new() -> Self {
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
                Shape::BezierCurve(bezier) => {
                    // For SVG, we'll use a path element to represent the bezier curve
                    // Format the path data
                    if bezier.points.is_empty() {
                        continue;
                    }

                    // Start the path with the first point
                    let mut path_data = format!("M {} {}", bezier.points[0].0, bezier.points[0].1);

                    // Add curve commands based on the number of points
                    match bezier.points.len() {
                        2 => {
                            // Linear curve - just draw a line to the second point
                            path_data.push_str(&format!(
                                " L {} {}",
                                bezier.points[1].0, bezier.points[1].1
                            ));
                        }
                        3 => {
                            // Quadratic bezier curve
                            path_data.push_str(&format!(
                                " Q {} {} {} {}",
                                bezier.points[1].0,
                                bezier.points[1].1,
                                bezier.points[2].0,
                                bezier.points[2].1
                            ));
                        }
                        4 => {
                            // Cubic bezier curve
                            path_data.push_str(&format!(
                                " C {} {} {} {} {} {}",
                                bezier.points[1].0,
                                bezier.points[1].1,
                                bezier.points[2].0,
                                bezier.points[2].1,
                                bezier.points[3].0,
                                bezier.points[3].1
                            ));
                        }
                        _ => {
                            // For more than 4 points, we'll approximate with multiple cubic curves
                            // This is a simplified approach - in a real implementation, you might
                            // want to use a more sophisticated curve fitting algorithm
                            for chunk in bezier.points[1..].chunks(3) {
                                if chunk.len() == 1 {
                                    // Linear to the point
                                    path_data
                                        .push_str(&format!(" L {} {}", chunk[0].0, chunk[0].1));
                                } else if chunk.len() == 2 {
                                    // Quadratic curve
                                    path_data.push_str(&format!(
                                        " Q {} {} {} {}",
                                        chunk[0].0, chunk[0].1, chunk[1].0, chunk[1].1
                                    ));
                                } else if chunk.len() == 3 {
                                    // Cubic curve
                                    path_data.push_str(&format!(
                                        " C {} {} {} {} {} {}",
                                        chunk[0].0,
                                        chunk[0].1,
                                        chunk[1].0,
                                        chunk[1].1,
                                        chunk[2].0,
                                        chunk[2].1
                                    ));
                                }
                            }
                        }
                    }

                    write!(file, r#"  <path d="{path_data}""#)?;

                    if let Some(stroke) = bezier.stroke {
                        write!(
                            file,
                            r#" stroke="{}" stroke-width="{}""#,
                            stroke.color, stroke.width
                        )?;
                    } else {
                        // Default stroke if none provided
                        write!(file, r#" stroke="black" stroke-width="1""#)?;
                    }

                    // Bezier curves are typically not filled, but we'll support it if specified
                    if let Some(color) = bezier.fill_color {
                        write!(file, r#" fill="{color}""#)?;
                    } else {
                        write!(file, r#" fill="none""#)?;
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
    use crate::builder::ShapeBuilder;
    use crate::shape::TextBuilder;
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
        let mut backend = SVGBackend::new();

        // Add text with special characters
        image.add(Shape::Text(
            TextBuilder::new()
                .with_pos(10.0, 10.0)
                .with_text("Hello & welcome <to> \"SVG\"")
                .with_font_size(12.0)
                .build()
                .unwrap(),
        ));

        // Create a temporary file path
        let path = Path::new("test_output.svg");

        // Render the image
        assert!(image.save(path, &mut backend).is_ok());

        // Clean up
        let _ = std::fs::remove_file(path);
    }
}
