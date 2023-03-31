//! Euclidean distance metric.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Asin, Atan2, Length, Normalize,
};

use crate::prelude::{Distance, Field, Normal, Uv};

/// Euclidian distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct EuclideanMetric;

impl<Dim> Field<Dim, Distance> for EuclideanMetric
where
    Dim: Length,
{
    fn field(&self, _attr: Distance, p: Dim) -> f32 {
        p.length()
    }
}

impl<Dim> Field<Dim, Normal<Dim>> for EuclideanMetric
where
    Dim: Default + PartialEq + Normalize,
{
    fn field(&self, _attr: Normal<Dim>, p: Dim) -> Dim {
        let d = Dim::default();
        if p == d {
            return d;
        }

        p.normalize()
    }
}

impl Field<f32, Uv> for EuclideanMetric {
    fn field(&self, _attr: Uv, p: f32) -> Vec2 {
        Vec2::new(p, 0.0)
    }
}

impl Field<Vec2, Uv> for EuclideanMetric {
    fn field(&self, _attr: Uv, p: Vec2) -> Vec2 {
        Vec2::new((p.x.atan2(p.y) / core::f32::consts::TAU) + 0.5, p.length())
    }
}

impl Field<Vec3, Uv> for EuclideanMetric {
    fn field(&self, _attr: Uv, p: Vec3) -> Vec2 {
        Vec2::new(
            (p.x.atan2(p.z) / core::f32::consts::TAU) + 0.5,
            (p.y.asin() / core::f32::consts::PI) + 0.5,
        )
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::glam::{Vec2, Vec3};

    use crate::prelude::{BoundTester, EuclideanMetric};

    #[test]
    fn test_euclidean_metric_2d() {
        assert!(BoundTester::<Vec2, EuclideanMetric>::default().is_field());
    }

    #[test]
    fn test_euclidean_metric_3d() {
        assert!(BoundTester::<Vec3, EuclideanMetric>::default().is_field());
    }
}
