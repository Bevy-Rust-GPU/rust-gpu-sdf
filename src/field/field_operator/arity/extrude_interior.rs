//! Extrude a 2D distance field into 3D, using its interior distance to determine depth.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::Field;

use crate::prelude::{Distance, Field, FieldOperator, Normal, Operator, Uv};

/// Extrude a 2D distance field into 3D, using its interior distance to determine depth.
/// NOTE: The present implementation is a bound, not a field
/// TODO: Refactor to use a 1D FieldFunction to describe Z curvature
#[derive(Default, Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct ExtrudeInteriorOp {
    pub depth: f32,
}

impl<Sdf> FieldOperator<Sdf, Distance<Vec2>> for ExtrudeInteriorOp
where
    Sdf: Field<Distance<f32>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: &Vec2,
    ) -> <Distance<Vec2> as crate::prelude::Attribute>::Output {
        let d = sdf.field(&p.x);
        let w = Vec2::new(d, p.y.abs() + d.min(0.0) * self.depth);
        let exterior = w.max(Vec2::ZERO).length();
        let interior = w.x.max(w.y).min(0.0);
        interior + exterior
    }
}

impl<Sdf> FieldOperator<Sdf, Distance<Vec3>> for ExtrudeInteriorOp
where
    Sdf: Field<Distance<Vec2>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: &Vec3,
    ) -> <Distance<Vec3> as crate::prelude::Attribute>::Output {
        let d = sdf.field(&p.truncate());
        let w = Vec2::new(d, p.z.abs() + d.min(0.0) * self.depth);
        let exterior = w.max(Vec2::ZERO).length();
        let interior = w.x.max(w.y).min(0.0);
        interior + exterior
    }
}

impl<Sdf> FieldOperator<Sdf, Normal<Vec2>> for ExtrudeInteriorOp
where
    Sdf: Field<Normal<f32>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: &Vec2,
    ) -> <Normal<Vec2> as crate::prelude::Attribute>::Output {
        let d = sdf.field(&p.x);
        Vec2::new(d, 1.0).normalize()
    }
}

impl<Sdf> FieldOperator<Sdf, Normal<Vec3>> for ExtrudeInteriorOp
where
    Sdf: Field<Normal<Vec2>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: &Vec3,
    ) -> <Normal<Vec3> as crate::prelude::Attribute>::Output {
        let d = sdf.field(&p.truncate());
        d.extend(1.0).normalize()
    }
}

impl<Sdf> FieldOperator<Sdf, Uv<Vec3>> for ExtrudeInteriorOp
where
    Sdf: crate::prelude::Field<Uv<Vec2>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: &Vec3,
    ) -> <Uv<Vec3> as crate::prelude::Attribute>::Output {
        sdf.field(&p.truncate())
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
    use crate::{
        prelude::{BoundTester, Circle, ExtrudeInterior, Point},
        test_op_attrs_3d,
    };

    #[test]
    fn test_extrude_interior() {
        assert!(BoundTester::<ExtrudeInterior::<Circle>>::default().is_field_3d());
    }

    test_op_attrs_3d!(ExtrudeInterior::<Point>);
}
