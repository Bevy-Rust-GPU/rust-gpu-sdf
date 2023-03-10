//! Rotate a distance field.
use rust_gpu_bridge::prelude::{Quat, Vec2, Vec3};

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Rotate a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct RotateOp2d {
    pub rotation: f32,
}

impl SignedDistanceOperator<Vec2> for RotateOp2d {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec2) -> f32
    where
        Sdf: SignedDistanceField<Vec2>,
    {
        sdf.distance(Vec2::from_angle(-self.rotation).rotate(p))
    }
}

/// Rotate a 3D distance field.
pub type Rotate2d<Sdf> = Operator<Sdf, RotateOp2d, Vec2>;

/// Rotate a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct RotateOp3d {
    pub rotation: Quat,
}

impl SignedDistanceOperator<Vec3> for RotateOp3d {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        sdf.distance(self.rotation.inverse() * p)
    }
}

/// Rotate a distance field.
pub type Rotate3d<Sdf> = Operator<Sdf, RotateOp3d, Vec3>;
