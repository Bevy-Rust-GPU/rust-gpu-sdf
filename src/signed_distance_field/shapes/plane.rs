//! A plane.
use rust_gpu_bridge::prelude::{Vec2, Vec3};
use type_fields::Field;

use crate::signed_distance_field::{Distance, SignedDistanceField};

/// A plane.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
pub struct Plane<Dim> {
    pub dir: Dim,
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

impl SignedDistanceField<Vec2, Distance> for Plane<Vec2> {
    fn evaluate(&self, p: Vec2) -> Distance {
        assert!(self.dir.is_normalized(), "Plane dir must be normalized");
        p.dot(-self.dir).into()
    }
}

impl SignedDistanceField<Vec3, Distance> for Plane<Vec3> {
    fn evaluate(&self, p: Vec3) -> Distance {
        assert!(self.dir.is_normalized(), "Plane dir must be normalized");
        p.dot(-self.dir).into()
    }
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::Vec3;
    use type_fields::field::Field;

    use super::Plane;

    #[test]
    pub fn test_plane() {
        Plane::<_>::default().with(Plane::dir, Vec3::default());
    }
}
