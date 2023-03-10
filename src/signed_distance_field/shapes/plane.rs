//! A plane.
use rust_gpu_bridge::prelude::Vec3;

use crate::signed_distance_field::SignedDistanceField;

/// A plane.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Plane {
    dir: Vec3,
}

impl Default for Plane {
    fn default() -> Self {
        Plane { dir: Vec3::Y }
    }
}

impl SignedDistanceField<Vec3> for Plane {
    fn distance(&self, p: Vec3) -> f32 {
        assert!(self.dir.is_normalized(), "Plane dir must be normalized");
        p.dot(-self.dir)
    }
}

