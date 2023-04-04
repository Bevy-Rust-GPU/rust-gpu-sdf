//! A plane.
use core::ops::Neg;

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::Field;

use crate::prelude::{AttrDistance, Field, AttrNormal, items::position::Position, Distance, Normal};

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

impl Field<AttrDistance<f32>> for Plane<f32> {
    fn field(&self, p: &Position<f32>) -> Distance {
        assert!(self.dir.abs() == 1.0, "Plane dir must be normalized");
        (**p * -self.dir).into()
    }
}

impl Field<AttrDistance<Vec2>> for Plane<Vec2> {
    fn field(&self, p: &Position<Vec2>) -> Distance {
        assert!(self.dir.is_normalized(), "Plane dir must be normalized");
        p.dot(-self.dir).into()
    }
}

impl Field<AttrDistance<Vec3>> for Plane<Vec3> {
    fn field(&self, p: &Position<Vec3>) -> Distance {
        assert!(self.dir.is_normalized(), "Plane dir must be normalized");
        p.dot(-self.dir).into()
    }
}

impl<Dim> Field<AttrNormal<Dim>> for Plane<Dim>
where
    Dim: Clone + Neg<Output = Dim>,
{
    fn field(&self, p: &Position<Dim>) -> Normal<Dim> {
        self.dir.clone().neg().into()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::prelude::BoundTester;

    use super::Plane;

    #[test]
    pub fn test_plane_2d() {
        assert!(BoundTester::<Plane<_>>::default().is_field_2d())
    }

    #[test]
    pub fn test_plane_3d() {
        assert!(BoundTester::<Plane<_>>::default().is_field_3d())
    }
}
