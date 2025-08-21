//! Circle shape builder

use crate::{
    builder::{BuildError, ShapeBuilder},
    shape::Circle,
    Color, Stroke,
};
use std::convert::Into;

/// Builder for creating circles with validation
#[derive(Debug, Default)]
pub struct CircleBuilder {
    x: Option<f64>,
    y: Option<f64>,
    radius: Option<f64>,
    diameter: Option<f64>,
    area: Option<f64>,
    fill_color: Option<Color>,
    stroke: Option<Stroke>,
}

impl CircleBuilder {
    /// Create a new circle builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the center position of the circle
    pub fn with_center<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.x = Some(x.into());
        self.y = Some(y.into());
        self
    }

    /// Set the radius of the circle
    pub fn with_radius<T>(mut self, radius: T) -> Self
    where
        T: Into<f64>,
    {
        self.radius = Some(radius.into());
        self
    }

    /// Set the diameter of the circle
    pub fn with_diameter<T>(mut self, diameter: T) -> Self
    where
        T: Into<f64>,
    {
        self.diameter = Some(diameter.into());
        self
    }

    /// Set the area of the circle, from which the radius will be calculated
    pub fn from_area<T>(mut self, area: T) -> Self
    where
        T: Into<f64>,
    {
        self.area = Some(area.into());
        self
    }

    /// Set the fill color of the circle
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set the stroke of the circle
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

impl ShapeBuilder<Circle> for CircleBuilder {
    /// Build the circle, performing validation and calculating derived values
    fn build(self) -> Result<Circle, BuildError> {
        let x = self.x.unwrap_or(0.0);
        let y = self.y.unwrap_or(0.0);

        // Check for conflicting properties
        let specified_props = [
            self.radius.is_some(),
            self.diameter.is_some(),
            self.area.is_some(),
        ]
        .iter()
        .filter(|&&b| b)
        .count();

        if specified_props > 1 {
            return Err(BuildError::ConflictingProperties(
                "Only one of radius, diameter, or area can be specified".to_string(),
            ));
        }

        // Calculate radius based on provided property
        let radius = if let Some(radius) = self.radius {
            if radius.is_nan() || radius <= 0.0 {
                return Err(BuildError::InvalidValue(
                    "Radius must be a positive number".to_string(),
                ));
            }
            radius
        } else if let Some(diameter) = self.diameter {
            if diameter.is_nan() || diameter <= 0.0 {
                return Err(BuildError::InvalidValue(
                    "Diameter must be a positive number".to_string(),
                ));
            }
            diameter / 2.0
        } else if let Some(area) = self.area {
            if area.is_nan() || area <= 0.0 {
                return Err(BuildError::InvalidValue(
                    "Area must be a positive number".to_string(),
                ));
            }
            (area / std::f64::consts::PI).sqrt()
        } else {
            return Err(BuildError::MissingRequiredField(
                "One of radius, diameter, or area must be specified".to_string(),
            ));
        };

        Ok(Circle {
            x,
            y,
            radius,
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
    fn test_circle_builder_with_radius() {
        let circle = CircleBuilder::new()
            .with_center(10.0, 20.0)
            .with_radius(5.0)
            .build()
            .expect("Failed to build circle");

        assert_eq!(circle.x, 10.0);
        assert_eq!(circle.y, 20.0);
        assert_eq!(circle.radius, 5.0);
    }

    #[test]
    fn test_circle_builder_with_diameter() {
        let circle = CircleBuilder::new()
            .with_center(10.0, 20.0)
            .with_diameter(10.0)
            .build()
            .expect("Failed to build circle");

        assert_eq!(circle.x, 10.0);
        assert_eq!(circle.y, 20.0);
        assert_eq!(circle.radius, 5.0);
    }

    #[test]
    fn test_circle_builder_from_area() {
        let area = 25.0 * std::f64::consts::PI; // Area of circle with radius 5
        let circle = CircleBuilder::new()
            .with_center(10.0, 20.0)
            .from_area(area)
            .build()
            .expect("Failed to build circle");

        assert_eq!(circle.x, 10.0);
        assert_eq!(circle.y, 20.0);
        // Using epsilon comparison for floating point values
        assert!((circle.radius - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_circle_builder_defaults() {
        let circle = CircleBuilder::new()
            .with_radius(5.0)
            .build()
            .expect("Failed to build circle");

        assert_eq!(circle.x, 0.0);
        assert_eq!(circle.y, 0.0);
        assert_eq!(circle.radius, 5.0);
    }

    #[test]
    fn test_circle_builder_with_fill_and_stroke() {
        let color = Color::RED;
        let stroke = Stroke::new().with_size(2.0).with_color(Color::BLUE);

        let circle = CircleBuilder::new()
            .with_radius(5.0)
            .with_fill_color(color)
            .with_stroke(stroke)
            .build()
            .expect("Failed to build circle");

        assert_eq!(circle.fill_color, Some(color));
        assert_eq!(circle.stroke, Some(stroke));
    }

    #[test]
    fn test_circle_builder_conflicting_properties() {
        let result = CircleBuilder::new()
            .with_radius(5.0)
            .with_diameter(10.0)
            .build();

        assert!(matches!(
            result,
            Err(BuildError::ConflictingProperties(_))
        ));
    }

    #[test]
    fn test_circle_builder_missing_required_field() {
        let result = CircleBuilder::new().build();

        assert!(matches!(
            result,
            Err(BuildError::MissingRequiredField(_))
        ));
    }

    #[test]
    fn test_circle_builder_invalid_radius() {
        let result = CircleBuilder::new().with_radius(-1.0).build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }

    #[test]
    fn test_circle_builder_invalid_diameter() {
        let result = CircleBuilder::new().with_diameter(0.0).build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }

    #[test]
    fn test_circle_builder_invalid_area() {
        let result = CircleBuilder::new().from_area(f64::NAN).build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }
}