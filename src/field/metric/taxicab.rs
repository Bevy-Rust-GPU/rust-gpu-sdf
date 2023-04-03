//! Taxicab distance metric.

use core::ops::Add;

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs, Sign, Normalize
};

use crate::prelude::{Distance, Field, Normal};

/// Taxicab distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct TaxicabMetric;

impl Field<Distance<f32>> for TaxicabMetric {
    fn field(&self, p: f32) -> f32 {
        p.abs()
    }
}

impl Field<Distance<Vec2>> for TaxicabMetric {
    fn field(&self, p: Vec2) -> f32 {
        p.x.abs().add(p.y.abs())
    }
}

impl Field<Distance<Vec3>> for TaxicabMetric {
    fn field(&self, p: Vec3) -> f32 {
        p.x.abs().add(p.y.abs()).add(p.z.abs())
    }
}

impl<Dim> Field<Normal<Dim>> for TaxicabMetric
where
    Dim: Sign + Normalize,
{
    fn field(&self, p: Dim) -> Dim {
        p.sign().normalize()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::prelude::{BoundTester, TaxicabMetric};

    #[test]
    #[should_panic]
    pub fn test_taxicab_metric_2d() {
        assert!(BoundTester::<TaxicabMetric>::default().is_field_2d());
    }

    #[test]
    #[should_panic]
    pub fn test_taxicab_metric_3d() {
        assert!(BoundTester::<TaxicabMetric>::default().is_field_3d());
    }
}
