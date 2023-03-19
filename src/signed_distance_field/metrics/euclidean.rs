//! Euclidean distance metric.

use rust_gpu_bridge::prelude::{Asin, Atan2, Length, Normalize, Vec2, Vec3};

use crate::{
    prelude::Normal,
    signed_distance_field::{attributes::uv::Uv, Distance, DistanceFunction},
};

/// Euclidian distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct EuclideanMetric;

impl<Dim> DistanceFunction<Dim, Distance> for EuclideanMetric
where
    Dim: Length,
{
    fn evaluate(&self, p: Dim) -> Distance {
        p.length().into()
    }
}

impl<Dim> DistanceFunction<Dim, Normal<Dim>> for EuclideanMetric
where
    Dim: Normalize,
{
    fn evaluate(&self, p: Dim) -> Normal<Dim> {
        p.normalize().into()
    }
}

impl DistanceFunction<Vec2, Uv> for EuclideanMetric {
    fn evaluate(&self, p: Vec2) -> Uv {
        Vec2::new((p.x.atan2(p.y) / core::f32::consts::TAU) + 0.5, p.length()).into()
    }
}

impl DistanceFunction<Vec3, Uv> for EuclideanMetric {
    fn evaluate(&self, p: Vec3) -> Uv {
        Vec2::new(
            (p.x.atan2(p.z) / core::f32::consts::TAU) + 0.5,
            (p.y.asin() / core::f32::consts::PI) + 0.5,
        )
        .into()
    }
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::{Vec2, Vec3};

    use crate::{
        prelude::BoundChecker, signed_distance_field::metrics::euclidean::EuclideanMetric,
    };

    #[test]
    fn test_euclidean_metric_2d() {
        assert!(BoundChecker::<Vec2, EuclideanMetric>::default().is_field());
    }

    #[test]
    fn test_euclidean_metric_3d() {
        assert!(BoundChecker::<Vec3, EuclideanMetric>::default().is_field());
    }
}
