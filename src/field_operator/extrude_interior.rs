//! Extrude a 2D distance field into 3D, using its interior distance to determine depth.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::Field;

use crate::prelude::{Distance, FieldFunction};

use super::{FieldOperator, Operator};

/// Extrude a 2D distance field into 3D, using its interior distance to determine depth.
/// NOTE: The present implementation is a bound, not a field
#[derive(Default, Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct ExtrudeInteriorOp {
    pub depth: f32,
}

impl<Sdf> FieldOperator<Sdf, Vec2, Distance> for ExtrudeInteriorOp
where
    Sdf: FieldFunction<f32, Distance>,
{
    fn operator(
        &self,
        attr: Distance,
        sdf: &Sdf,
        p: Vec2,
    ) -> <Distance as crate::prelude::Attribute>::Type {
        let d = sdf.evaluate(attr, p.x);
        let w = Vec2::new(d, p.y.abs() + d.min(0.0) * self.depth);
        let exterior = w.max(Vec2::ZERO).length();
        let interior = w.x.max(w.y).min(0.0);
        interior + exterior
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Distance> for ExtrudeInteriorOp
where
    Sdf: FieldFunction<Vec2, Distance>,
{
    fn operator(
        &self,
        attr: Distance,
        sdf: &Sdf,
        p: Vec3,
    ) -> <Distance as crate::prelude::Attribute>::Type {
        let d = sdf.evaluate(attr, p.truncate());
        let w = Vec2::new(d, p.z.abs() + d.min(0.0) * self.depth);
        let exterior = w.max(Vec2::ZERO).length();
        let interior = w.x.max(w.y).min(0.0);
        interior + exterior
    }
}

/// Uniformly scale a distance field.
pub type ExtrudeInterior<Sdf> = Operator<ExtrudeInteriorOp, Sdf>;

impl<Sdf> ExtrudeInterior<Sdf> {
    pub fn depth(&mut self) -> &mut f32 {
        self.op().depth()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use rust_gpu_bridge::glam::Vec3;

    use crate::{prelude::BoundChecker, signed_distance_field::shapes::composite::Circle};

    use super::Sweep;

    #[test]
    fn test_sweep() {
        assert!(BoundChecker::<Vec3, ExtrudeInteriorOp::<Circle, Circle>>::default().is_field());
    }
}
