//! Extrude a 2D distance field into 3D.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::Field;

use crate::prelude::{Distance, FieldFunction};

use super::{FieldOperator, Operator};

/// Extrude a 2D distance field into 3D.
#[derive(Default, Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct ExtrudeOp {
    pub axis: Vec3,
    pub depth: f32,
}

impl<Sdf> FieldOperator<Sdf, Vec2, Distance> for ExtrudeOp
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
        let w = Vec2::new(d, p.y.abs() - self.depth);
        w.x.max(w.y).min(0.0) + w.max(Vec2::ZERO).length()
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Distance> for ExtrudeOp
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
        let w = Vec2::new(d, p.z.abs() - self.depth);
        w.x.max(w.y).min(0.0) + w.max(Vec2::ZERO).length()
    }
}

/// Uniformly scale a distance field.
pub type Extrude<Sdf> = Operator<ExtrudeOp, Sdf>;

impl<Sdf> Extrude<Sdf> {
    pub fn axis(&mut self) -> &mut Vec3 {
        self.op().axis()
    }

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
        assert!(BoundChecker::<Vec3, ExtrudeOp::<Circle, Circle>>::default().is_field());
    }
}
