//! Extrude a 2D distance field into 3D, using its interior distance to determine depth.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::Field;

use crate::prelude::{
    items::position::Position, AttrDistance, Field, FieldOperator, AttrNormal, Operator, AttrUv,
};

/// Extrude a 2D distance field into 3D, using its interior distance to determine depth.
/// NOTE: The present implementation is a bound, not a field
/// TODO: Refactor to use a 1D FieldFunction to describe Z curvature
#[derive(Default, Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct ExtrudeInteriorOp {
    pub depth: f32,
}

impl<Sdf> FieldOperator<Sdf, AttrDistance<Vec2>> for ExtrudeInteriorOp
where
    Sdf: Field<AttrDistance<f32>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: &Position<Vec2>,
    ) -> <AttrDistance<Vec2> as crate::prelude::Attribute>::Output {
        let d = *sdf.field(&p.x.into());
        let w = Vec2::new(d, p.y.abs() + d.min(0.0) * self.depth);
        let exterior = w.max(Vec2::ZERO).length();
        let interior = w.x.max(w.y).min(0.0);
        (interior + exterior).into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrDistance<Vec3>> for ExtrudeInteriorOp
where
    Sdf: Field<AttrDistance<Vec2>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: &Position<Vec3>,
    ) -> <AttrDistance<Vec3> as crate::prelude::Attribute>::Output {
        let d = *sdf.field(&p.truncate().into());
        let w = Vec2::new(d, p.z.abs() + d.min(0.0) * self.depth);
        let exterior = w.max(Vec2::ZERO).length();
        let interior = w.x.max(w.y).min(0.0);
        (interior + exterior).into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrNormal<Vec2>> for ExtrudeInteriorOp
where
    Sdf: Field<AttrNormal<f32>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec2>) -> <AttrNormal<Vec2> as crate::prelude::Attribute>::Output {
        let d = *sdf.field(&p.x.into());
        Vec2::new(d, 1.0).normalize().into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrNormal<Vec3>> for ExtrudeInteriorOp
where
    Sdf: Field<AttrNormal<Vec2>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec3>) -> <AttrNormal<Vec3> as crate::prelude::Attribute>::Output {
        let d = *sdf.field(&p.truncate().into());
        d.extend(1.0).normalize().into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrUv<Vec3>> for ExtrudeInteriorOp
where
    Sdf: crate::prelude::Field<AttrUv<Vec2>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec3>) -> <AttrUv<Vec3> as crate::prelude::Attribute>::Output {
        sdf.field(&p.truncate().into())
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
