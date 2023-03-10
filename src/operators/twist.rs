//! Twist a distance field around an arbitrary axis.

use rust_gpu_bridge::prelude::{Quat, Vec3};

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Twist a distance field around an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TwistOp {
    pub axis_pos: Vec3,
    pub axis_rot: Vec3,
    pub k: f32,
}

impl Default for TwistOp {
    fn default() -> Self {
        TwistOp {
            axis_pos: Vec3::Y,
            axis_rot: Vec3::Y,
            k: 1.0,
        }
    }
}

impl SignedDistanceOperator<Vec3> for TwistOp {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        let q = Quat::from_axis_angle(self.axis_rot, self.k * self.axis_pos.dot(p)) * p;
        return sdf.distance(q);
    }
}

/// Twist a distance field around an arbitrary axis.
pub type Twist<Sdf> = Operator<Sdf, TwistOp, Vec3>;
