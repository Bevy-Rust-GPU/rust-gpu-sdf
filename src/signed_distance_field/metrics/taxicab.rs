//! Taxicab distance metric.

use core::ops::Add;

use rust_gpu_bridge::prelude::{Vec2, Vec3};

use crate::signed_distance_field::{Distance, SignedDistanceField};

/// Taxicab distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaxicabMetric;

impl SignedDistanceField<f32, Distance> for TaxicabMetric {
    fn evaluate(&self, p: f32) -> Distance {
        p.abs().into()
    }
}

impl SignedDistanceField<Vec2, Distance> for TaxicabMetric {
    fn evaluate(&self, p: Vec2) -> Distance {
        p.x.abs().add(p.y.abs()).into()
    }
}

impl SignedDistanceField<Vec3, Distance> for TaxicabMetric {
    fn evaluate(&self, p: Vec3) -> Distance {
        p.x.abs().add(p.y.abs()).add(p.z.abs()).into()
    }
}
