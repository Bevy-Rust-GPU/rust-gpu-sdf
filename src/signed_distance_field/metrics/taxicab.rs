//! Taxicab distance metric.

use core::ops::Add;

use rust_gpu_bridge::prelude::{Abs, Normalize, Sign, Vec2, Vec3};

use crate::signed_distance_field::{attributes::normal::Normal, Distance, DistanceFunction};

/// Taxicab distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct TaxicabMetric;

impl DistanceFunction<f32, Distance> for TaxicabMetric {
    fn evaluate(&self, p: f32) -> Distance {
        p.abs().into()
    }
}

impl DistanceFunction<Vec2, Distance> for TaxicabMetric {
    fn evaluate(&self, p: Vec2) -> Distance {
        p.x.abs().add(p.y.abs()).into()
    }
}

impl DistanceFunction<Vec3, Distance> for TaxicabMetric {
    fn evaluate(&self, p: Vec3) -> Distance {
        p.x.abs().add(p.y.abs()).add(p.z.abs()).into()
    }
}

impl<Dim> DistanceFunction<Dim, Normal<Dim>> for TaxicabMetric
where
    Dim: Sign,
{
    fn evaluate(&self, p: Dim) -> Normal<Dim> {
        p.sign().into()
    }
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::{Vec2, Vec3};

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
