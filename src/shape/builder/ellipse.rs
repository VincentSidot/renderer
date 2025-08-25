//! Ellipse shape builder

use crate::{
    Color, Stroke,
    builder::{BuildError, ShapeBuilder},
    shape::Ellipse,
};
use std::convert::Into;

/// Builder for creating ellipses with validation
#[derive(Debug, Default)]
pub struct EllipseBuilder {
    x: Option<f64>,
    y: Option<f64>,
    radius_x: Option<f64>,
    radius_y: Option<f64>,
    fill_color: Option<Color>,
    stroke: Option<Stroke>,
}

impl EllipseBuilder {
    /// Create a new ellipse builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the center position of the ellipse
    pub fn with_center<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.x = Some(x.into());
        self.y = Some(y.into());
        self
    }

    /// Set the radii of the ellipse
    pub fn with_radii<T, U>(mut self, radius_x: T, radius_y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.radius_x = Some(radius_x.into());
        self.radius_y = Some(radius_y.into());
        self
    }

    /// Set the fill color of the ellipse
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set the stroke of the ellipse
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

impl ShapeBuilder<Ellipse> for EllipseBuilder {
    /// Build the ellipse, performing validation
    fn build(self) -> Result<Ellipse, BuildError> {
        let x = self.x.unwrap_or(0.0);
        let y = self.y.unwrap_or(0.0);
        let radius_x = self.radius_x.ok_or_else(|| {
            BuildError::MissingRequiredField("Ellipse radius_x must be specified".to_string())
        })?;
        let radius_y = self.radius_y.ok_or_else(|| {
            BuildError::MissingRequiredField("Ellipse radius_y must be specified".to_string())
        })?;

        // Validate radii
        if radius_x.is_nan() || radius_x <= 0.0 {
            return Err(BuildError::InvalidValue(
                "Radius_x must be a positive number".to_string(),
            ));
        }

        if radius_y.is_nan() || radius_y <= 0.0 {
            return Err(BuildError::InvalidValue(
                "Radius_y must be a positive number".to_string(),
            ));
        }

        Ok(Ellipse {
            x,
            y,
            radius_x,
            radius_y,
            fill_color: self.fill_color,
            stroke: self.stroke,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color::Color, stroke::Stroke};

    #[test]
    fn test_ellipse_builder_success() {
        let ellipse = EllipseBuilder::new()
            .with_center(10.0, 20.0)
            .with_radii(30.0, 20.0)
            .build()
            .expect("Failed to build ellipse");

        assert_eq!(ellipse.x(), 10.0);
        assert_eq!(ellipse.y(), 20.0);
        assert_eq!(ellipse.radius_x(), 30.0);
        assert_eq!(ellipse.radius_y(), 20.0);
    }

    #[test]
    fn test_ellipse_builder_defaults() {
        let ellipse = EllipseBuilder::new()
            .with_radii(30.0, 20.0)
            .build()
            .expect("Failed to build ellipse");

        assert_eq!(ellipse.x(), 0.0);
        assert_eq!(ellipse.y(), 0.0);
        assert_eq!(ellipse.radius_x(), 30.0);
        assert_eq!(ellipse.radius_y(), 20.0);
    }

    #[test]
    fn test_ellipse_builder_with_fill_and_stroke() {
        let color = Color::RED;
        let stroke = Stroke::new().with_size(2.0).with_color(Color::BLUE);

        let ellipse = EllipseBuilder::new()
            .with_radii(30.0, 20.0)
            .with_fill_color(color)
            .with_stroke(stroke)
            .build()
            .expect("Failed to build ellipse");

        assert_eq!(ellipse.fill_color(), Some(&color));
        assert_eq!(ellipse.stroke(), Some(&stroke));
    }

    #[test]
    fn test_ellipse_builder_missing_radius_x() {
        let result = EllipseBuilder::new().with_radii(30.0, 20.0).build();
        // This should succeed since both radii are provided
        assert!(result.is_ok());

        // Test with missing radius_x
        let result = EllipseBuilder::new().with_center(10.0, 20.0).build();
        assert!(matches!(result, Err(BuildError::MissingRequiredField(_))));
    }

    #[test]
    fn test_ellipse_builder_invalid_radius_x() {
        let result = EllipseBuilder::new().with_radii(-1.0, 20.0).build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }

    #[test]
    fn test_ellipse_builder_invalid_radius_y() {
        let result = EllipseBuilder::new().with_radii(30.0, 0.0).build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }

    #[test]
    fn test_ellipse_builder_nan_values() {
        let result = EllipseBuilder::new().with_radii(f64::NAN, 20.0).build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }
}
