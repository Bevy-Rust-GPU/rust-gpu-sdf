//! Reflect a distance field about an arbitrary axis.

use rust_gpu_bridge::{prelude::Vec3, reflect::Reflect as ReflectTrait};

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Reflect a distance field about an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ReflectOp {
    pub axis: Vec3,
}

impl Default for ReflectOp {
    fn default() -> Self {
        ReflectOp { axis: Vec3::X }
    }
}

impl SignedDistanceOperator<Vec3> for ReflectOp {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        assert!(
            self.axis.is_normalized(),
            "ReflectOp axis must be normalized"
        );
        let q = if p.dot(self.axis) >= 0.0 {
            p
        } else {
            p.reflect(self.axis)
        };
        sdf.distance(q)
    }
}

/// Reflect a distance field about an arbitrary axis.
pub type Reflect<Sdf> = Operator<Sdf, ReflectOp, Vec3>;

