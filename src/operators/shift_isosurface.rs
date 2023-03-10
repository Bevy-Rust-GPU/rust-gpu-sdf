//! Add an arbitrary radius to a distance field.

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Add an arbitrary radius to a distance field.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct ShiftIsosurfaceOp {
    pub delta: f32,
}

impl Default for ShiftIsosurfaceOp {
    fn default() -> Self {
        ShiftIsosurfaceOp { delta: 1.0 }
    }
}

impl<Dim> SignedDistanceOperator<Dim> for ShiftIsosurfaceOp {
    fn operator<Sdf>(&self, sdf: Sdf, p: Dim) -> f32
    where
        Sdf: SignedDistanceField<Dim>,
    {
        sdf.distance(p) - self.delta
    }
}

/// Add an arbitrary radius to a distance field.
pub type ShiftIsosurface<Sdf, Dim> = Operator<Sdf, ShiftIsosurfaceOp, Dim>;
