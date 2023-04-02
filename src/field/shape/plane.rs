//! A plane.
use core::ops::Neg;

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::Field;

use crate::prelude::{Distance, Field, Normal};

/// A plane.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
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

impl Field<f32, Distance> for Plane<f32> {
    fn field(&self, _attr: Distance, p: f32) -> f32 {
        assert!(self.dir.abs() == 1.0, "Plane dir must be normalized");
        p * -self.dir
    }
}

impl Field<Vec2, Distance> for Plane<Vec2> {
    fn field(&self, _attr: Distance, p: Vec2) -> f32 {
        assert!(self.dir.is_normalized(), "Plane dir must be normalized");
        p.dot(-self.dir)
    }
}

impl Field<Vec3, Distance> for Plane<Vec3> {
    fn field(&self, _attr: Distance, p: Vec3) -> f32 {
        assert!(self.dir.is_normalized(), "Plane dir must be normalized");
        p.dot(-self.dir)
    }
}

impl<Dim> Field<Dim, Normal<Dim>> for Plane<Dim>
where
    Dim: Clone + Neg<Output = Dim>,
{
    fn field(&self, _attr: Normal<Dim>, _p: Dim) -> Dim {
        -self.dir.clone()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::glam::{Vec2, Vec3};

    use crate::prelude::BoundTester;

    use super::Plane;

    #[test]
    pub fn test_plane_2d() {
        assert!(BoundTester::<Vec2, Plane<_>>::default().is_field())
    }

    #[test]
    pub fn test_plane_3d() {
        assert!(BoundTester::<Vec3, Plane<_>>::default().is_field())
    }
}
