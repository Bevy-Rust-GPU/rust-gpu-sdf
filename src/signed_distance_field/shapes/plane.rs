//! A plane.
use core::ops::Neg;

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::Field;

use crate::signed_distance_field::{attributes::normal::Normal, Distance, DistanceFunction};

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

impl DistanceFunction<f32, Distance> for Plane<f32> {
    fn evaluate(&self, attr: Distance, p: f32) -> f32 {
        assert!(self.dir.abs() == 1.0, "Plane dir must be normalized");
        p * -self.dir
    }
}

impl DistanceFunction<Vec2, Distance> for Plane<Vec2> {
    fn evaluate(&self, attr: Distance, p: Vec2) -> f32 {
        assert!(self.dir.is_normalized(), "Plane dir must be normalized");
        p.dot(-self.dir)
    }
}

impl DistanceFunction<Vec3, Distance> for Plane<Vec3> {
    fn evaluate(&self, attr: Distance, p: Vec3) -> f32 {
        assert!(self.dir.is_normalized(), "Plane dir must be normalized");
        p.dot(-self.dir)
    }
}

impl<Dim> DistanceFunction<Dim, Normal<Dim>> for Plane<Dim>
where
    Dim: Clone + Neg<Output = Dim>,
{
    fn evaluate(&self, attr: Normal<Dim>, _p: Dim) -> Dim {
        -self.dir.clone()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::glam::{Vec2, Vec3};

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
