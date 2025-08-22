//! Line shape builder

use crate::{
    builder::{BuildError, ShapeBuilder},
    shape::Line,
    Stroke,
};
use std::convert::Into;

/// Builder for creating lines with validation
#[derive(Debug, Default)]
pub struct LineBuilder {
    x1: Option<f64>,
    y1: Option<f64>,
    x2: Option<f64>,
    y2: Option<f64>,
    stroke: Option<Stroke>,
}

impl LineBuilder {
    /// Create a new line builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the start position of the line
    pub fn with_start<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.x1 = Some(x.into());
        self.y1 = Some(y.into());
        self
    }

    /// Set the end position of the line
    pub fn with_end<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.x2 = Some(x.into());
        self.y2 = Some(y.into());
        self
    }

    /// Set the stroke of the line
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

impl ShapeBuilder<Line> for LineBuilder {
    /// Build the line, performing validation
    fn build(self) -> Result<Line, BuildError> {
        let x1 = self.x1.ok_or_else(|| {
            BuildError::MissingRequiredField("Line start x position must be specified".to_string())
        })?;
        let y1 = self.y1.ok_or_else(|| {
            BuildError::MissingRequiredField("Line start y position must be specified".to_string())
        })?;
        let x2 = self.x2.ok_or_else(|| {
            BuildError::MissingRequiredField("Line end x position must be specified".to_string())
        })?;
        let y2 = self.y2.ok_or_else(|| {
            BuildError::MissingRequiredField("Line end y position must be specified".to_string())
        })?;

        // Check if start and end points are the same
        if (x1 - x2).abs() < f64::EPSILON && (y1 - y2).abs() < f64::EPSILON {
            return Err(BuildError::InvalidValue(
                "Line start and end points cannot be the same".to_string(),
            ));
        }

        Ok(Line {
            x1,
            y1,
            x2,
            y2,
            stroke: self.stroke,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stroke::Stroke;

    #[test]
    fn test_line_builder_success() {
        let line = LineBuilder::new()
            .with_start(0.0, 0.0)
            .with_end(10.0, 10.0)
            .build()
            .expect("Failed to build line");

        assert_eq!(line.x1(), 0.0);
        assert_eq!(line.y1(), 0.0);
        assert_eq!(line.x2(), 10.0);
        assert_eq!(line.y2(), 10.0);
    }

    #[test]
    fn test_line_builder_with_stroke() {
        let stroke = Stroke::new().with_size(2.0).with_color(crate::Color::RED);

        let line = LineBuilder::new()
            .with_start(0.0, 0.0)
            .with_end(10.0, 10.0)
            .with_stroke(stroke)
            .build()
            .expect("Failed to build line");

        assert_eq!(line.stroke(), Some(&stroke));
    }

    #[test]
    fn test_line_builder_missing_start_x() {
        let result = LineBuilder::new().with_end(10.0, 10.0).build();

        assert!(matches!(
            result,
            Err(BuildError::MissingRequiredField(_))
        ));
    }

    #[test]
    fn test_line_builder_missing_end_y() {
        let result = LineBuilder::new().with_start(0.0, 0.0).build();

        assert!(matches!(
            result,
            Err(BuildError::MissingRequiredField(_))
        ));
    }

    #[test]
    fn test_line_builder_same_points() {
        let result = LineBuilder::new()
            .with_start(5.0, 5.0)
            .with_end(5.0, 5.0)
            .build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }
}