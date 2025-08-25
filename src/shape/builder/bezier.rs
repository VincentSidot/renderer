//! Bezier curve shape builder

use crate::{
    Color, Stroke,
    builder::{BuildError, ShapeBuilder},
    shape::BezierCurve,
};
use std::convert::Into;

/// Builder for creating bezier curves with validation
#[derive(Debug, Default)]
pub struct BezierCurveBuilder {
    points: Vec<(f64, f64)>,
    stroke: Option<Stroke>,
    fill_color: Option<Color>,
}

impl BezierCurveBuilder {
    /// Create a new bezier curve builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a control point to the bezier curve
    pub fn add_point<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.points.push((x.into(), y.into()));
        self
    }

    /// Set multiple control points for the bezier curve
    pub fn with_points(mut self, points: Vec<(f64, f64)>) -> Self {
        self.points = points;
        self
    }

    /// Set the stroke of the bezier curve
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }

    /// Set the fill color of the bezier curve
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }
}

impl ShapeBuilder<BezierCurve> for BezierCurveBuilder {
    /// Build the bezier curve, performing validation
    fn build(self) -> Result<BezierCurve, BuildError> {
        // Validate that we have at least 2 points for a curve
        if self.points.len() < 2 {
            return Err(BuildError::InvalidValue(
                "A bezier curve must have at least 2 control points".to_string(),
            ));
        }

        // Check for NaN values
        for (i, (x, y)) in self.points.iter().enumerate() {
            if x.is_nan() || y.is_nan() {
                return Err(BuildError::InvalidValue(format!(
                    "Point {} contains NaN values",
                    i
                )));
            }
        }

        Ok(BezierCurve {
            points: self.points,
            stroke: self.stroke,
            fill_color: self.fill_color,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color::Color, stroke::Stroke};

    #[test]
    fn test_bezier_curve_builder_success() {
        let bezier = BezierCurveBuilder::new()
            .add_point(0.0, 0.0)
            .add_point(50.0, 50.0)
            .add_point(100.0, 0.0)
            .build()
            .expect("Failed to build bezier curve");

        assert_eq!(bezier.points().len(), 3);
        assert_eq!(bezier.points()[0], (0.0, 0.0));
        assert_eq!(bezier.points()[1], (50.0, 50.0));
        assert_eq!(bezier.points()[2], (100.0, 0.0));
    }

    #[test]
    fn test_bezier_curve_builder_with_points() {
        let points = vec![(0.0, 0.0), (50.0, 50.0), (100.0, 0.0)];
        let bezier = BezierCurveBuilder::new()
            .with_points(points)
            .build()
            .expect("Failed to build bezier curve");

        assert_eq!(bezier.points().len(), 3);
    }

    #[test]
    fn test_bezier_curve_builder_with_stroke_and_fill() {
        let color = Color::RED;
        let stroke = Stroke::new().with_size(2.0).with_color(Color::BLUE);
        let points = vec![(0.0, 0.0), (50.0, 50.0), (100.0, 0.0)];

        let bezier = BezierCurveBuilder::new()
            .with_points(points)
            .with_fill_color(color)
            .with_stroke(stroke)
            .build()
            .expect("Failed to build bezier curve");

        assert_eq!(bezier.fill_color(), Some(&color));
        assert_eq!(bezier.stroke(), Some(&stroke));
    }

    #[test]
    fn test_bezier_curve_builder_not_enough_points() {
        let result = BezierCurveBuilder::new().add_point(0.0, 0.0).build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }

    #[test]
    fn test_bezier_curve_builder_nan_values() {
        let result = BezierCurveBuilder::new()
            .add_point(0.0, 0.0)
            .add_point(f64::NAN, 0.0)
            .add_point(100.0, 100.0)
            .build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }
}
