//! Extrude a 2D distance field into 3D.

use rust_gpu_bridge::prelude::{Vec2, Vec3};
use type_fields::Field;

use crate::signed_distance_field::SignedDistanceField;

/// Extrude a 2D distance field into 3D.
#[derive(Debug, Default, Copy, Clone, PartialEq, Field)]
pub struct Extrude<Sdf>
where
    Sdf: SignedDistanceField<Vec2>,
{
    pub sdf: Sdf,
    pub axis: Vec3,
    pub depth: f32,
}

impl<Sdf> SignedDistanceField<Vec3> for Extrude<Sdf>
where
    Sdf: SignedDistanceField<Vec2>,
{
    fn distance(&self, p: Vec3) -> f32 {
        let d = self.sdf.distance(p.truncate());
        let w = Vec2::new(d, p.z.abs() - self.depth);
        w.x.max(w.y).min(0.0) + w.max(Vec2::ZERO).length()
    }
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::Vec3;
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Square;

    use super::Extrude;

    #[test]
    fn test_extrude() {
        Extrude::<Square>::default()
            .with(Extrude::<()>::SDF, Square::default())
            .with(Extrude::<()>::AXIS, Vec3::default())
            .with(Extrude::<()>::DEPTH, f32::default());
    }
}
