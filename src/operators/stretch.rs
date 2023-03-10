//! Stretch a shape along an arbitrary axis, preserving exterior geometry as caps.

use rust_gpu_bridge::prelude::{Vec2, Vec3};

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Extrude a shape by an arbitrary distance along an arbitrary axis, preserving exterior geometry as caps.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StretchDistOp<Dim> {
    pub dir: Dim,
    pub dist: f32,
}

impl Default for StretchDistOp<Vec2> {
    fn default() -> Self {
        StretchDistOp {
            dir: Vec2::X,
            dist: 1.0,
        }
    }
}

impl Default for StretchDistOp<Vec3> {
    fn default() -> Self {
        StretchDistOp {
            dir: Vec3::X,
            dist: 1.0,
        }
    }
}

impl SignedDistanceOperator<Vec2> for StretchDistOp<Vec2> {
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

impl SignedDistanceOperator<Vec3> for StretchDistOp<Vec3> {
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
pub type StretchDist<Sdf, Dim> = Operator<Sdf, StretchDistOp<Dim>, Dim>;

/// Extrude a shape infinitely along an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StretchInfiniteOp<Dim> {
    pub dir: Dim,
}

impl Default for StretchInfiniteOp<Vec2> {
    fn default() -> Self {
        StretchInfiniteOp { dir: Vec2::X }
    }
}

impl Default for StretchInfiniteOp<Vec3> {
    fn default() -> Self {
        StretchInfiniteOp { dir: Vec3::X }
    }
}

impl SignedDistanceOperator<Vec2> for StretchInfiniteOp<Vec2> {
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

impl SignedDistanceOperator<Vec3> for StretchInfiniteOp<Vec3> {
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
pub type StretchInfinite<Sdf, Dim> = Operator<Sdf, StretchInfiniteOp<Dim>, Dim>;
