//! Add an arbitrary radius to a distance field.

use rust_gpu_bridge::prelude::Vec3;

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

impl SignedDistanceOperator<Vec3> for RoundOp {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        sdf.distance(p) - self.radius
    }
}

/// Add an arbitrary radius to a distance field.
pub type Round<Sdf> = Operator<Sdf, RoundOp, Vec3>;
