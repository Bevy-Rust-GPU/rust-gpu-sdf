//! Rotate a distance field.
use rust_gpu_bridge::prelude::{Quat, Vec2, Vec3};

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Rotate a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Rotate2dOp {
    pub rotation: f32,
}

impl SignedDistanceOperator<Vec2> for Rotate2dOp {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> f32
    where
        Sdf: SignedDistanceField<Vec2>,
    {
        sdf.distance(Vec2::from_angle(-self.rotation).rotate(p))
    }
}

/// Rotate a 3D distance field.
pub type Rotate2d<Sdf> = Operator<Sdf, Rotate2dOp, Vec2>;

/// Rotate a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Rotate3dOp {
    pub rotation: Quat,
}

impl SignedDistanceOperator<Vec3> for Rotate3dOp {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        sdf.distance(self.rotation.inverse() * p)
    }
}

/// Rotate a distance field.
pub type Rotate3d<Sdf> = Operator<Sdf, Rotate3dOp, Vec3>;
