//! Twist a distance field around an arbitrary axis.

use rust_gpu_bridge::prelude::{Quat, Vec2, Vec3};

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Twist a distance field around an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TwistOp<Dim> {
    pub axis_pos: Dim,
    pub axis_rot: Dim,
    pub k: f32,
}

impl Default for TwistOp<Vec2> {
    fn default() -> Self {
        TwistOp {
            axis_pos: Vec2::Y,
            axis_rot: Vec2::Y,
            k: 1.0,
        }
    }
}

impl Default for TwistOp<Vec3> {
    fn default() -> Self {
        TwistOp {
            axis_pos: Vec3::Y,
            axis_rot: Vec3::Y,
            k: 1.0,
        }
    }
}

impl SignedDistanceOperator<Vec2> for TwistOp<Vec2> {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec2) -> f32
    where
        Sdf: SignedDistanceField<Vec2>,
    {
        let q = Vec2::from_angle(self.k * self.axis_pos.dot(p)).rotate(p);
        return sdf.distance(q);
    }
}

impl SignedDistanceOperator<Vec3> for TwistOp<Vec3> {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        let q = Quat::from_axis_angle(self.axis_rot, self.k * self.axis_pos.dot(p)) * p;
        return sdf.distance(q);
    }
}

/// Twist a distance field around an arbitrary axis.
pub type Twist<Sdf, Dim> = Operator<Sdf, TwistOp<Dim>, Dim>;
