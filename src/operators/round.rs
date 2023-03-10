//! Add an arbitrary radius to a distance field.

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Add an arbitrary radius to a distance field.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct RoundOp {
    pub radius: f32,
}

impl Default for RoundOp {
    fn default() -> Self {
        RoundOp { radius: 1.0 }
    }
}

impl<Dim> SignedDistanceOperator<Dim> for RoundOp {
    fn operator<Sdf>(&self, sdf: Sdf, p: Dim) -> f32
    where
        Sdf: SignedDistanceField<Dim>,
    {
        sdf.distance(p) - self.radius
    }
}

/// Add an arbitrary radius to a distance field.
pub type Round<Sdf, Dim> = Operator<Sdf, RoundOp, Dim>;
