//! Euclidean distance metric.

use rust_gpu_bridge::prelude::{Vec2, Vec3};

use crate::{
    prelude::Normal,
    signed_distance_field::{Distance, SignedDistanceField},
};

/// Euclidian distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EuclideanMetric;

impl SignedDistanceField<f32, Distance> for EuclideanMetric {
    fn evaluate(&self, p: f32) -> Distance {
        p.abs().into()
    }
}

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

impl SignedDistanceField<f32, Normal<f32>> for EuclideanMetric {
    fn evaluate(&self, p: f32) -> Normal<f32> {
        p.signum().into()
    }
}

impl SignedDistanceField<Vec2, Normal<Vec2>> for EuclideanMetric {
    fn evaluate(&self, p: Vec2) -> Normal<Vec2> {
        p.normalize().into()
    }
}

impl SignedDistanceField<Vec3, Normal<Vec3>> for EuclideanMetric {
    fn evaluate(&self, p: Vec3) -> Normal<Vec3> {
        p.normalize().into()
    }
}
