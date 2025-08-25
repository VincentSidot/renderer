//! Builder module for the renderer
//!
//! This module provides builder patterns for creating images and shapes with validation.

use std::fmt;

/// Error type for builder validation failures
#[derive(Debug, Clone, PartialEq)]
pub enum BuildError {
    /// A required field is missing
    MissingRequiredField(String),
    /// An invalid value was provided (negative, zero, NaN, etc.)
    InvalidValue(String),
    /// Conflicting properties were specified
    ConflictingProperties(String),
    /// A feature required for this operation is not available
    FeatureUnavailable(String),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::MissingRequiredField(field) => {
                write!(f, "Missing required field: {}", field)
            }
            BuildError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            BuildError::ConflictingProperties(msg) => {
                write!(f, "Conflicting properties: {}", msg)
            }
            BuildError::FeatureUnavailable(feature) => {
                write!(f, "Feature unavailable: {}", feature)
            }
        }
    }
}

impl std::error::Error for BuildError {}

/// Trait for all shape builders
pub trait ShapeBuilder<T> {
    /// Build the shape, performing validation and returning either the shape or a BuildError
    fn build(self) -> Result<T, BuildError>;
}
