//! Stretch a shape along an arbitrary axis, preserving exterior geometry as caps.

use core::ops::{Mul, Sub};

use rust_gpu_bridge::prelude::{Dot, Length, Vec2, Vec3};
use type_fields::Field;

use crate::{
    prelude::{Operator, DistanceFunction, SignedDistanceOperator},
};

/// Extrude a shape infinitely along an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
#[repr(C)]
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

impl<Sdf, Dim, Out> SignedDistanceOperator<Sdf, Dim, Out> for StretchInfiniteOp<Dim>
where
    Sdf: DistanceFunction<Dim, Out>,
    Dim: Clone + Mul<f32, Output = Dim> + Sub<Dim, Output = Dim> + Length + Dot,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Out {
        assert!(
            self.dir.clone().length() == 1.0,
            "ExtrudeInfiniteOp dir must be normalized"
        );
        let q = p.clone() - self.dir.clone() * p.dot(self.dir.clone());
        sdf.evaluate(q)
    }
}

/// Extrude a shape infinitely along an arbitrary axis.
pub type StretchInfinite<Dim, Sdf> = Operator<StretchInfiniteOp<Dim>, Sdf>;

impl<Dim, Sdf> StretchInfinite<Dim, Sdf> {
    pub fn dir(&mut self) -> &mut Dim {
        &mut self.op.dir
    }
}

/// Extrude a shape by an arbitrary distance along an arbitrary axis, preserving exterior geometry as caps.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct StretchDistOp<Dim> {
    pub dir: Dim,
    pub dist: f32,
}

impl Default for StretchDistOp<f32> {
    fn default() -> Self {
        StretchDistOp {
            dir: 1.0,
            dist: 1.0,
        }
    }
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

impl<Sdf, Dim, Out> SignedDistanceOperator<Sdf, Dim, Out> for StretchDistOp<Dim>
where
    Sdf: DistanceFunction<Dim, Out>,
    Dim: Clone + Mul<f32, Output = Dim> + Sub<Dim, Output = Dim> + Dot,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Out {
        let q =
            p.clone() - (self.dir.clone() * p.dot(self.dir.clone()).clamp(-self.dist, self.dist));
        sdf.evaluate(q)
    }
}

/// Extrude a shape by an arbitrary distance along an arbitrary axis, preserving exterior geometry as caps.
pub type StretchDist<Dim, Sdf> = Operator<StretchDistOp<Dim>, Sdf>;

impl<Dim, Sdf> StretchDist<Dim, Sdf> {
    pub fn dir(&mut self) -> &mut Dim {
        &mut self.op.dir
    }

    pub fn dist(&mut self) -> &mut f32 {
        &mut self.op.dist
    }
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::Vec3;
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Cube;

    use super::{StretchDist, StretchInfinite};

    #[test]
    fn test_stretch_infinite() {
        StretchInfinite::<_, Cube>::default().with(StretchInfinite::dir, Vec3::default());
    }

    #[test]
    fn test_stretch_dist() {
        StretchDist::<_, Cube>::default()
            .with(StretchDist::dir, Vec3::default())
            .with(StretchDist::dist, f32::default());
    }
}
