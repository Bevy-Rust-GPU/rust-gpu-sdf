//! Taxicab distance metric.

use core::ops::Add;

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs, Sign,
};

use crate::signed_distance_field::{attributes::normal::Normal, Distance, DistanceFunction};

/// Taxicab distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct TaxicabMetric;

impl DistanceFunction<f32, Distance> for TaxicabMetric {
    fn evaluate(&self, attr: Distance, p: f32) -> f32 {
        p.abs()
    }
}

impl DistanceFunction<Vec2, Distance> for TaxicabMetric {
    fn evaluate(&self, attr: Distance, p: Vec2) -> f32 {
        p.x.abs().add(p.y.abs())
    }
}

impl DistanceFunction<Vec3, Distance> for TaxicabMetric {
    fn evaluate(&self, attr: Distance, p: Vec3) -> f32 {
        p.x.abs().add(p.y.abs()).add(p.z.abs())
    }
}

impl<Dim> DistanceFunction<Dim, Normal<Dim>> for TaxicabMetric
where
    Dim: Sign,
{
    fn evaluate(&self, attr: Normal<Dim>, p: Dim) -> Dim {
        p.sign()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::glam::{Vec2, Vec3};

    use crate::{prelude::BoundChecker, signed_distance_field::metrics::taxicab::TaxicabMetric};

    #[test]
    #[should_panic]
    pub fn test_taxicab_metric_2d() {
        assert!(BoundChecker::<Vec2, TaxicabMetric>::default().is_field());
    }

    #[test]
    #[should_panic]
    pub fn test_taxicab_metric_3d() {
        assert!(BoundChecker::<Vec3, TaxicabMetric>::default().is_field());
    }
}
