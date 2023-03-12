//! Chebyshev distance metric.

use rust_gpu_bridge::prelude::{Vec2, Vec3};

use crate::signed_distance_field::{Distance, SignedDistanceField};

/// Chebyshev distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChebyshevMetric;

impl SignedDistanceField<Vec2, Distance> for ChebyshevMetric {
    fn evaluate(&self, p: Vec2) -> Distance {
        p.x.abs().max(p.y.abs()).into()
    }
}

impl SignedDistanceField<Vec3, Distance> for ChebyshevMetric {
    fn evaluate(&self, p: Vec3) -> Distance {
        p.x.abs().max(p.y.abs()).max(p.z.abs()).into()
    }
}
