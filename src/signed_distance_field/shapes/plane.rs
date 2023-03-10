//! A plane.
use rust_gpu_bridge::prelude::{Vec2, Vec3};

use crate::signed_distance_field::SignedDistanceField;

/// A plane.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Plane<Dim> {
    dir: Dim,
}

impl Default for Plane<Vec2> {
    fn default() -> Self {
        Plane { dir: Vec2::Y }
    }
}

impl Default for Plane<Vec3> {
    fn default() -> Self {
        Plane { dir: Vec3::Y }
    }
}

impl SignedDistanceField<Vec2> for Plane<Vec2> {
    fn distance(&self, p: Vec2) -> f32 {
        assert!(self.dir.is_normalized(), "Plane dir must be normalized");
        p.dot(-self.dir)
    }
}

impl SignedDistanceField<Vec3> for Plane<Vec3> {
    fn distance(&self, p: Vec3) -> f32 {
        assert!(self.dir.is_normalized(), "Plane dir must be normalized");
        p.dot(-self.dir)
    }
}
