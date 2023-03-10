//! Operators for repeating distance fields across a domain.

use rust_gpu_bridge::{modulo::Mod, prelude::Vec3};

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Repeat a distance field infinitely in one or more axes.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RepeatInfiniteOp {
    pub period: Vec3,
}

impl Default for RepeatInfiniteOp {
    fn default() -> Self {
        RepeatInfiniteOp { period: Vec3::ONE }
    }
}

impl SignedDistanceOperator<Vec3> for RepeatInfiniteOp {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        let q = (p + 0.5 * self.period).modulo(self.period) - (0.5 * self.period);
        sdf.distance(q)
    }
}

/// Repeat a distance field infinitely in one or more axes.
pub type RepeatInfinite<Sdf> = Operator<Sdf, RepeatInfiniteOp, Vec3>;

/// Repeat a distance field a set number of times in one or more axes.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RepeatCountOp {
    pub period: Vec3,
    pub count: Vec3,
}

impl Default for RepeatCountOp {
    fn default() -> Self {
        RepeatCountOp {
            period: Vec3::ONE,
            count: Vec3::ONE * 1.0,
        }
    }
}

impl SignedDistanceOperator<Vec3> for RepeatCountOp {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        let q = p - self.period * (p / self.period).round().clamp(-self.count, self.count);
        sdf.distance(q)
    }
}

/// Repeat a distance field a set number of times in one or more axes.
pub type RepeatCount<Sdf> = Operator<Sdf, RepeatCountOp, Vec3>;
