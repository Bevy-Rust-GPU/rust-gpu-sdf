//! Reflect a distance field about an arbitrary axis.

use rust_gpu_bridge::{
    prelude::{Vec2, Vec3},
    reflect::Reflect as ReflectTrait,
};

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Reflect a distance field about an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ReflectOp<Dim> {
    pub axis: Dim,
}

impl Default for ReflectOp<Vec2> {
    fn default() -> Self {
        ReflectOp { axis: Vec2::X }
    }
}

impl Default for ReflectOp<Vec3> {
    fn default() -> Self {
        ReflectOp { axis: Vec3::X }
    }
}

impl SignedDistanceOperator<Vec2> for ReflectOp<Vec2> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> f32
    where
        Sdf: SignedDistanceField<Vec2>,
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

impl SignedDistanceOperator<Vec3> for ReflectOp<Vec3> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> f32
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
pub type Reflect<Sdf, Dim> = Operator<Sdf, ReflectOp<Dim>, Dim>;