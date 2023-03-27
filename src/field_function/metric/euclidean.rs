//! Euclidean distance metric.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Asin, Atan2, Cross, Length, Normalize,
};

use crate::prelude::{Distance, FieldFunction, Normal, Tangent, Uv};

/// Euclidian distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct EuclideanMetric;

impl<Dim> FieldFunction<Dim, Distance> for EuclideanMetric
where
    Dim: Length,
{
    fn evaluate(&self, _attr: Distance, p: Dim) -> f32 {
        p.length()
    }
}

impl<Dim> FieldFunction<Dim, Normal<Dim>> for EuclideanMetric
where
    Dim: Normalize,
{
    fn evaluate(&self, _attr: Normal<Dim>, p: Dim) -> Dim {
        p.normalize()
    }
}

impl FieldFunction<f32, Uv> for EuclideanMetric {
    fn evaluate(&self, _attr: Uv, p: f32) -> Vec2 {
        Vec2::new(p, 0.0)
    }
}

impl FieldFunction<Vec2, Uv> for EuclideanMetric {
    fn evaluate(&self, _attr: Uv, p: Vec2) -> Vec2 {
        Vec2::new((p.x.atan2(p.y) / core::f32::consts::TAU) + 0.5, p.length())
    }
}

impl FieldFunction<Vec3, Uv> for EuclideanMetric {
    fn evaluate(&self, _attr: Uv, p: Vec3) -> Vec2 {
        Vec2::new(
            (p.x.atan2(p.z) / core::f32::consts::TAU) + 0.5,
            (p.y.asin() / core::f32::consts::PI) + 0.5,
        )
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::glam::{Vec2, Vec3};

    use crate::prelude::{BoundChecker, EuclideanMetric};

    #[test]
    fn test_euclidean_metric_2d() {
        assert!(BoundChecker::<Vec2, EuclideanMetric>::default().is_field());
    }

    #[test]
    fn test_euclidean_metric_3d() {
        assert!(BoundChecker::<Vec3, EuclideanMetric>::default().is_field());
    }
}
