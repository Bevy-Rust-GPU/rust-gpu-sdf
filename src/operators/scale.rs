//! Uniformly scale a distance field.

use rust_gpu_bridge::prelude::Vec3;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Uniformly scale a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct ScaleOp {
    scale: f32,
}

impl SignedDistanceOperator<Vec3> for ScaleOp {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        sdf.distance(p / self.scale) * self.scale
    }
}

/// Uniformly scale a distance field.
pub type Scale<Sdf> = Operator<Sdf, ScaleOp, Vec3>;

