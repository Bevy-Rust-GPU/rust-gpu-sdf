//! Chebyshev distance metric.

use rust_gpu_bridge::prelude::{Vec2, Vec3};

use crate::signed_distance_field::SignedDistanceField;

/// Chebyshev distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChebyshevMetric;

impl SignedDistanceField<Vec2, f32> for ChebyshevMetric {
    fn evaluate(&self, p: Vec2) -> f32 {
        p.x.abs().max(p.y.abs())
    }
}

impl SignedDistanceField<Vec3, f32> for ChebyshevMetric {
    fn evaluate(&self, p: Vec3) -> f32 {
        p.x.abs().max(p.y.abs()).max(p.z.abs())
    }
}
