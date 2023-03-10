//! Rotate a distance field.
use rust_gpu_bridge::prelude::{Quat, Vec3};

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Rotate a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct RotateOp {
    pub rotation: Quat,
}

impl SignedDistanceOperator<Vec3> for RotateOp {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        sdf.distance(self.rotation.inverse() * p)
    }
}

/// Rotate a distance field.
pub type Rotate<Sdf> = Operator<Sdf, RotateOp, Vec3>;
