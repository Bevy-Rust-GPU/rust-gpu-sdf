//! A plane.
use rust_gpu_bridge::prelude::{Vec2, Vec3, Abs};
use type_fields::Field;

use crate::signed_distance_field::{Distance, SignedDistanceField};

/// A plane.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct Plane<Dim> {
    pub dir: Dim,
}

impl Default for Plane<f32> {
    fn default() -> Self {
        Plane { dir: 1.0 }
    }
}

impl Default for Plane<Vec2> {
    fn default() -> Self {
        Plane { dir: Vec2::Y }
    }
}

impl Default for Plane<Vec3> {
    fn default() -> Self {
        Plane { dir: Vec3::Y }
    }
}

impl SignedDistanceField<f32, Distance> for Plane<f32> {
    fn evaluate(&self, p: f32) -> Distance {
        assert!(self.dir.abs() == 1.0, "Plane dir must be normalized");
        (p * -self.dir).into()
    }
}

impl SignedDistanceField<Vec2, Distance> for Plane<Vec2> {
    fn evaluate(&self, p: Vec2) -> Distance {
        assert!(self.dir.is_normalized(), "Plane dir must be normalized");
        p.dot(-self.dir).into()
    }
}

impl SignedDistanceField<Vec3, Distance> for Plane<Vec3> {
    fn evaluate(&self, p: Vec3) -> Distance {
        assert!(self.dir.is_normalized(), "Plane dir must be normalized");
        p.dot(-self.dir).into()
    }
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::{Vec2, Vec3};

    use crate::prelude::BoundChecker;

    use super::Plane;

    #[test]
    pub fn test_plane_2d() {
        assert!(BoundChecker::<Vec2, Plane<_>>::default().is_field())
    }

    #[test]
    pub fn test_plane_3d() {
        assert!(BoundChecker::<Vec3, Plane<_>>::default().is_field())
    }
}
