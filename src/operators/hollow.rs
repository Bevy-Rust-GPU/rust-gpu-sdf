//! Convert a solid shape into a hollow one with an infinitely thin surface.

use rust_gpu_bridge::prelude::Vec3;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Convert a solid shape into a hollow one with an infinitely thin surface.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct HollowOp {
    pub depth: f32,
}

impl Default for HollowOp {
    fn default() -> Self {
        HollowOp { depth: 1.0 }
    }
}

impl SignedDistanceOperator<Vec3> for HollowOp {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        sdf.distance(p).abs()
    }
}

/// Convert a solid shape into a hollow one with an infinitely thin surface.
pub type Hollow<Sdf> = Operator<Sdf, HollowOp, Vec3>;
