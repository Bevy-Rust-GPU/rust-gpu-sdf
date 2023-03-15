//! Extrude a 2D distance field into 3D.

use rust_gpu_bridge::prelude::{Vec2, Vec3};
use type_fields::Field;

use crate::signed_distance_field::{Distance, SignedDistanceField};

/// Extrude a 2D distance field into 3D.
#[derive(Debug, Default, Copy, Clone, PartialEq, Field)]
pub struct Extrude<Sdf> {
    pub sdf: Sdf,
    pub axis: Vec3,
    pub depth: f32,
}

impl<Sdf> SignedDistanceField<Vec2, Distance> for Extrude<Sdf>
where
    Sdf: SignedDistanceField<f32, Distance>,
{
    fn evaluate(&self, p: Vec2) -> Distance {
        let d = self.sdf.evaluate(p.x);
        let w = Vec2::new(*d, p.y.abs() - self.depth);
        Distance(w.x.max(w.y).min(0.0) + w.max(Vec2::ZERO).length())
    }
}

impl<Sdf> SignedDistanceField<Vec3, Distance> for Extrude<Sdf>
where
    Sdf: SignedDistanceField<Vec2, Distance>,
{
    fn evaluate(&self, p: Vec3) -> Distance {
        let d = self.sdf.evaluate(p.truncate());
        let w = Vec2::new(*d, p.z.abs() - self.depth);
        Distance(w.x.max(w.y).min(0.0) + w.max(Vec2::ZERO).length())
    }
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::Vec3;

    use crate::{prelude::BoundChecker, signed_distance_field::shapes::composite::Square};

    use super::Extrude;

    #[test]
    fn test_extrude() {
        assert!(BoundChecker::<Vec3, Extrude::<Square>>::default().is_field());
    }
}
