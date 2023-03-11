//! Shift the isosurface of a distance field by a given amount.

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Shift the isosurface of a distance field by a given amount.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct IsosurfaceOp {
    pub delta: f32,
}

impl Default for IsosurfaceOp {
    fn default() -> Self {
        IsosurfaceOp { delta: 1.0 }
    }
}

impl<Dim> SignedDistanceOperator<Dim> for IsosurfaceOp {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Dim) -> f32
    where
        Sdf: SignedDistanceField<Dim>,
    {
        sdf.distance(p) - self.delta
    }
}

/// Add an arbitrary radius to a distance field.
pub type Isosurface<Sdf, Dim> = Operator<Sdf, IsosurfaceOp, Dim>;
