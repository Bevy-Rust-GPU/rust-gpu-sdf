//! Chebyshev distance metric.

use rust_gpu_bridge::prelude::Vec3;

use crate::signed_distance_field::SignedDistanceField;

/// Chebyshev distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChebyshevMetric;

impl SignedDistanceField<Vec3> for ChebyshevMetric {
    fn distance(&self, p: Vec3) -> f32 {
        p.x.abs().max(p.y.abs())
    }
}

