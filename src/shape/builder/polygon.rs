//! Polygon shape builder

use crate::{
    builder::{BuildError, ShapeBuilder},
    shape::Polygon,
    Color, Stroke,
};
use std::convert::Into;

/// Builder for creating polygons with validation
#[derive(Debug, Default)]
pub struct PolygonBuilder {
    points: Vec<(f64, f64)>,
    fill_color: Option<Color>,
    stroke: Option<Stroke>,
}

impl PolygonBuilder {
    /// Create a new polygon builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a point to the polygon
    pub fn add_point<T, U>(mut self, x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        self.points.push((x.into(), y.into()));
        self
    }

    /// Set multiple points for the polygon
    pub fn with_points(mut self, points: Vec<(f64, f64)>) -> Self {
        self.points = points;
        self
    }

    /// Set the fill color of the polygon
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set the stroke of the polygon
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

impl ShapeBuilder<Polygon> for PolygonBuilder {
    /// Build the polygon, performing validation
    fn build(self) -> Result<Polygon, BuildError> {
        // Validate that we have at least 3 points for a polygon
        if self.points.len() < 3 {
            return Err(BuildError::InvalidValue(
                "A polygon must have at least 3 points".to_string(),
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

        Ok(Polygon {
            points: self.points,
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
    fn test_polygon_builder_success() {
        let polygon = PolygonBuilder::new()
            .add_point(0.0, 0.0)
            .add_point(10.0, 0.0)
            .add_point(10.0, 10.0)
            .add_point(0.0, 10.0)
            .build()
            .expect("Failed to build polygon");

        assert_eq!(polygon.points().len(), 4);
        assert_eq!(polygon.points()[0], (0.0, 0.0));
        assert_eq!(polygon.points()[1], (10.0, 0.0));
        assert_eq!(polygon.points()[2], (10.0, 10.0));
        assert_eq!(polygon.points()[3], (0.0, 10.0));
    }

    #[test]
    fn test_polygon_builder_with_points() {
        let points = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)];
        let polygon = PolygonBuilder::new()
            .with_points(points)
            .build()
            .expect("Failed to build polygon");

        assert_eq!(polygon.points().len(), 4);
    }

    #[test]
    fn test_polygon_builder_with_fill_and_stroke() {
        let color = Color::RED;
        let stroke = Stroke::new().with_size(2.0).with_color(Color::BLUE);
        let points = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)];

        let polygon = PolygonBuilder::new()
            .with_points(points)
            .with_fill_color(color)
            .with_stroke(stroke)
            .build()
            .expect("Failed to build polygon");

        assert_eq!(polygon.fill_color(), Some(&color));
        assert_eq!(polygon.stroke(), Some(&stroke));
    }

    #[test]
    fn test_polygon_builder_not_enough_points() {
        let result = PolygonBuilder::new().add_point(0.0, 0.0).add_point(10.0, 0.0).build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }

    #[test]
    fn test_polygon_builder_nan_values() {
        let result = PolygonBuilder::new()
            .add_point(0.0, 0.0)
            .add_point(f64::NAN, 0.0)
            .add_point(10.0, 10.0)
            .build();

        assert!(matches!(result, Err(BuildError::InvalidValue(_))));
    }
}