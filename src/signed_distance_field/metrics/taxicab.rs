//! Taxicab distance metric.

use rust_gpu_bridge::prelude::Vec3;

use crate::signed_distance_field::SignedDistanceField;

/// Taxicab distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaxicabMetric;

impl SignedDistanceField<Vec3> for TaxicabMetric {
    fn distance(&self, p: Vec3) -> f32 {
        p.x.abs() + p.y.abs()
    }
}

