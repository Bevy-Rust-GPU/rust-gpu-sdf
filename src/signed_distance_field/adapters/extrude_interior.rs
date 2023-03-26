use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::Field;

use crate::prelude::{default, Distance, FieldFunction};

/// Extrude a 2D distance field into 3D, using its interior distance to determine depth.
/// NOTE: The present implementation is a bound, not a field
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
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

impl<Sdf> FieldFunction<Vec2, Distance> for ExtrudeInterior<Sdf>
where
    Sdf: FieldFunction<f32, Distance>,
{
    fn evaluate(&self, attr: Distance, p: Vec2) -> f32 {
        let d = self.sdf.evaluate(attr, p.x);
        let w = Vec2::new(d, p.y.abs() + d.min(0.0) * self.depth);
        let exterior = w.max(Vec2::ZERO).length();
        let interior = w.x.max(w.y).min(0.0);
        interior + exterior
    }
}

impl<Sdf> FieldFunction<Vec3, Distance> for ExtrudeInterior<Sdf>
where
    Sdf: FieldFunction<Vec2, Distance>,
{
    fn evaluate(&self, attr: Distance, p: Vec3) -> f32 {
        let d = self.sdf.evaluate(attr, p.truncate());
        let w = Vec2::new(d, p.z.abs() + d.min(0.0) * self.depth);
        let exterior = w.max(Vec2::ZERO).length();
        let interior = w.x.max(w.y).min(0.0);
        interior + exterior
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::glam::Vec3;

    use crate::{prelude::BoundChecker, signed_distance_field::shapes::composite::Circle};

    #[test]
    #[should_panic]
    pub fn extrude_interior() {
        assert!(BoundChecker::<Vec3, Circle>::default().is_field())
    }
}
