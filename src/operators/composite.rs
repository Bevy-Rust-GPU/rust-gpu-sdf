//! Operators composed from other operators.

use super::{hollow::Hollow, rotate::Rotate3d, isosurface::Isosurface, scale::Scale, translate::Translate};

/// Translate, rotate, and scale the wrapped SDF.
pub type Transform<Sdf, Dim> = Translate<Rotate3d<Scale<Sdf, Dim>>, Dim>;

/// Converts a solid shape into a hollow one with the given surface depth.
pub type Onion<Sdf, Dim> = Isosurface<Hollow<Sdf, Dim>, Dim>;
