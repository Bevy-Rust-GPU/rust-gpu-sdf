//! Extrude a shape along an arbitrary axis, preserving exterior geometry as caps.

use rust_gpu_bridge::prelude::{Vec2, Vec3};

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Extrude a shape by an arbitrary distance along an arbitrary axis, preserving exterior geometry as caps.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExtrudeDistOp<Dim> {
    pub dir: Dim,
    pub dist: f32,
}

impl Default for ExtrudeDistOp<Vec2> {
    fn default() -> Self {
        ExtrudeDistOp {
            dir: Vec2::X,
            dist: 1.0,
        }
    }
}

impl Default for ExtrudeDistOp<Vec3> {
    fn default() -> Self {
        ExtrudeDistOp {
            dir: Vec3::X,
            dist: 1.0,
        }
    }
}

impl SignedDistanceOperator<Vec2> for ExtrudeDistOp<Vec2> {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec2) -> f32
    where
        Sdf: SignedDistanceField<Vec2>,
    {
        assert!(
            self.dir.is_normalized(),
            "ExtrudeDistOp dir must be normalized"
        );
        let q = p - (p.dot(self.dir).clamp(-self.dist, self.dist) * self.dir);
        sdf.distance(q)
    }
}

impl SignedDistanceOperator<Vec3> for ExtrudeDistOp<Vec3> {
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
pub type ExtrudeDist<Sdf, Dim> = Operator<Sdf, ExtrudeDistOp<Dim>, Dim>;

/// Extrude a shape infinitely along an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExtrudeInfiniteOp<Dim> {
    pub dir: Dim,
}

impl Default for ExtrudeInfiniteOp<Vec2> {
    fn default() -> Self {
        ExtrudeInfiniteOp { dir: Vec2::X }
    }
}

impl Default for ExtrudeInfiniteOp<Vec3> {
    fn default() -> Self {
        ExtrudeInfiniteOp { dir: Vec3::X }
    }
}

impl SignedDistanceOperator<Vec2> for ExtrudeInfiniteOp<Vec2> {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec2) -> f32
    where
        Sdf: SignedDistanceField<Vec2>,
    {
        assert!(
            self.dir.is_normalized(),
            "ExtrudeInfiniteOp dir must be normalized"
        );
        let q = p - p.dot(self.dir) * self.dir;
        sdf.distance(q)
    }
}

impl SignedDistanceOperator<Vec3> for ExtrudeInfiniteOp<Vec3> {
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
pub type ExtrudeInfinite<Sdf, Dim> = Operator<Sdf, ExtrudeInfiniteOp<Dim>, Dim>;
