//! Extrude a shape along an arbitrary axis, preserving exterior geometry as caps.

use rust_gpu_bridge::prelude::Vec3;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Extrude a shape by an arbitrary distance along an arbitrary axis, preserving exterior geometry as caps.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExtrudeDistOp {
    pub dir: Vec3,
    pub dist: f32,
}

impl Default for ExtrudeDistOp {
    fn default() -> Self {
        ExtrudeDistOp {
            dir: Vec3::X,
            dist: 1.0,
        }
    }
}

impl SignedDistanceOperator<Vec3> for ExtrudeDistOp {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        assert!(
            self.dir.is_normalized(),
            "ExtrudeDistOp dir must be normalized"
        );
        let q = p - (p.dot(self.dir).clamp(-self.dist, self.dist) * self.dir);
        sdf.distance(q)
    }
}

/// Extrude a shape by an arbitrary distance along an arbitrary axis, preserving exterior geometry as caps.
pub type ExtrudeDist<Sdf> = Operator<Sdf, ExtrudeDistOp, Vec3>;

/// Extrude a shape infinitely along an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExtrudeInfiniteOp {
    pub dir: Vec3,
}

impl Default for ExtrudeInfiniteOp {
    fn default() -> Self {
        ExtrudeInfiniteOp { dir: Vec3::X }
    }
}

impl SignedDistanceOperator<Vec3> for ExtrudeInfiniteOp {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        assert!(
            self.dir.is_normalized(),
            "ExtrudeInfiniteOp dir must be normalized"
        );
        let q = p - p.dot(self.dir) * self.dir;
        sdf.distance(q)
    }
}

/// Extrude a shape infinitely along an arbitrary axis.
pub type ExtrudeInfinite<Sdf> = Operator<Sdf, ExtrudeInfiniteOp, Vec3>;

