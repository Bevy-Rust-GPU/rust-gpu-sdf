//! Euclidean distance metric.

use rust_gpu_bridge::prelude::{Vec2, Vec3};

use crate::signed_distance_field::SignedDistanceField;

/// Euclidian distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EuclideanMetric;

impl SignedDistanceField<Vec2, f32> for EuclideanMetric {
    fn evaluate(&self, p: Vec2) -> f32 {
        p.length()
    }
}

impl SignedDistanceField<Vec3, f32> for EuclideanMetric {
    fn evaluate(&self, p: Vec3) -> f32 {
        p.length()
    }
}

impl SignedDistanceField<Vec3, Vec3> for EuclideanMetric {
    fn evaluate(&self, p: Vec3) -> Vec3 {
        p.normalize()
    }
}
