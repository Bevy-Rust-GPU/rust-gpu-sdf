use rust_gpu_bridge::prelude::{Vec2, Vec3};
use type_fields::Field;

use crate::{
    default,
    signed_distance_field::{attributes::distance::Distance, SignedDistanceField},
};

/// Extrude a 2D distance field into 3D, using its interior distance to determine depth.
/// NOTE: The present implementation is a bound, not a field
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct ExtrudeInterior<Sdf> {
    sdf: Sdf,
    depth: f32,
}

impl<Sdf> Default for ExtrudeInterior<Sdf>
where
    Sdf: Default,
{
    fn default() -> Self {
        ExtrudeInterior {
            sdf: default(),
            depth: 1.0,
        }
    }
}

impl<Sdf> SignedDistanceField<Vec2, Distance> for ExtrudeInterior<Sdf>
where
    Sdf: SignedDistanceField<f32, Distance>,
{
    fn evaluate(&self, p: Vec2) -> Distance {
        let d = self.sdf.evaluate(p.x);
        let w = Vec2::new(*d, p.y.abs() + d.min(0.0) * self.depth);
        let exterior = w.max(Vec2::ZERO).length();
        let interior = w.x.max(w.y).min(0.0);
        Distance(interior + exterior)
    }
}

impl<Sdf> SignedDistanceField<Vec3, Distance> for ExtrudeInterior<Sdf>
where
    Sdf: SignedDistanceField<Vec2, Distance>,
{
    fn evaluate(&self, p: Vec3) -> Distance {
        let d = self.sdf.evaluate(p.truncate());
        let w = Vec2::new(*d, p.z.abs() + d.min(0.0) * self.depth);
        let exterior = w.max(Vec2::ZERO).length();
        let interior = w.x.max(w.y).min(0.0);
        Distance(interior + exterior)
    }
}
