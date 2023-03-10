//! A torus.
use rust_gpu_bridge::prelude::{Vec2, Vec3};

use crate::signed_distance_field::SignedDistanceField;

/// A torus.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Torus {
    radius_inner: f32,
    radius_outer: f32,
}

impl Default for Torus {
    fn default() -> Self {
        Torus {
            radius_outer: 0.75,
            radius_inner: 0.25,
        }
    }
}

impl SignedDistanceField<Vec3> for Torus {
    fn distance(&self, p: Vec3) -> f32 {
        let q = Vec2::new(Vec2::new(p.x, p.y).length() - self.radius_outer, p.z);
        return q.length() - self.radius_inner;
    }
}
