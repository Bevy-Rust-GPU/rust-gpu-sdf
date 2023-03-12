//! Euclidean distance metric.

use rust_gpu_bridge::prelude::{Vec2, Vec3};

use crate::{
    prelude::Normal,
    signed_distance_field::{Distance, SignedDistanceField},
};

/// Euclidian distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EuclideanMetric;

impl SignedDistanceField<Vec2, Distance> for EuclideanMetric {
    fn evaluate(&self, p: Vec2) -> Distance {
        p.length().into()
    }
}

impl SignedDistanceField<Vec3, Distance> for EuclideanMetric {
    fn evaluate(&self, p: Vec3) -> Distance {
        p.length().into()
    }
}

impl SignedDistanceField<Vec3, Normal> for EuclideanMetric {
    fn evaluate(&self, p: Vec3) -> Normal {
        p.normalize().into()
    }
}
