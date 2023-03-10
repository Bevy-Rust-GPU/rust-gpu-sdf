//! Stretch a shape along an arbitrary axis, preserving exterior geometry as caps.

use rust_gpu_bridge::prelude::{Vec2, Vec3};
use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

/// Extrude a shape infinitely along an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
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

impl SignedDistanceOperator<f32, Distance> for StretchInfiniteOp<f32> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: f32) -> Distance
    where
        Sdf: SignedDistanceField<f32, Distance>,
    {
        assert!(
            self.dir.abs() == 1.0,
            "ExtrudeInfiniteOp dir must be normalized"
        );
        let q = p - p * self.dir * self.dir;
        sdf.evaluate(q)
    }
}

impl SignedDistanceOperator<Vec2, Distance> for StretchInfiniteOp<Vec2> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> Distance
    where
        Sdf: SignedDistanceField<Vec2, Distance>,
    {
        assert!(
            self.dir.is_normalized(),
            "ExtrudeInfiniteOp dir must be normalized"
        );
        let q = p - p.dot(self.dir) * self.dir;
        sdf.evaluate(q)
    }
}

impl SignedDistanceOperator<Vec3, Distance> for StretchInfiniteOp<Vec3> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> Distance
    where
        Sdf: SignedDistanceField<Vec3, Distance>,
    {
        assert!(
            self.dir.is_normalized(),
            "ExtrudeInfiniteOp dir must be normalized"
        );
        let q = p - p.dot(self.dir) * self.dir;
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

impl SignedDistanceOperator<f32, Distance> for StretchDistOp<f32> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: f32) -> Distance
    where
        Sdf: SignedDistanceField<f32, Distance>,
    {
        assert!(
            self.dir.abs() == 1.0,
            "ExtrudeDistOp dir must be normalized"
        );
        let q = p - ((p * self.dir).clamp(-self.dist, self.dist) * self.dir);
        sdf.evaluate(q)
    }
}

impl SignedDistanceOperator<Vec2, Distance> for StretchDistOp<Vec2> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> Distance
    where
        Sdf: SignedDistanceField<Vec2, Distance>,
    {
        assert!(
            self.dir.is_normalized(),
            "ExtrudeDistOp dir must be normalized"
        );
        let q = p - (p.dot(self.dir).clamp(-self.dist, self.dist) * self.dir);
        sdf.evaluate(q)
    }
}

impl SignedDistanceOperator<Vec3, Distance> for StretchDistOp<Vec3> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> Distance
    where
        Sdf: SignedDistanceField<Vec3, Distance>,
    {
        assert!(
            self.dir.is_normalized(),
            "ExtrudeDistOp dir must be normalized"
        );
        let q = p - (p.dot(self.dir).clamp(-self.dist, self.dist) * self.dir);
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
