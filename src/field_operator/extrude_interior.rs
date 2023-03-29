//! Extrude a 2D distance field into 3D, using its interior distance to determine depth.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::Field;

use crate::prelude::{Distance, FieldFunction, Normal, Uv};

use super::{FieldOperator, Operator};

/// Extrude a 2D distance field into 3D, using its interior distance to determine depth.
/// NOTE: The present implementation is a bound, not a field
/// TODO: Refactor to use a 1D FieldFunction to describe Z curvature
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

impl<Sdf> FieldOperator<Sdf, Vec2, Normal<Vec2>> for ExtrudeInteriorOp
where
    Sdf: FieldFunction<f32, Normal<f32>>,
{
    fn operator(
        &self,
        _: Normal<Vec2>,
        sdf: &Sdf,
        p: Vec2,
    ) -> <Normal<Vec2> as crate::prelude::Attribute>::Type {
        let d = sdf.evaluate(Normal::<f32>::default(), p.x);
        Vec2::new(d, 1.0).normalize()
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Normal<Vec3>> for ExtrudeInteriorOp
where
    Sdf: FieldFunction<Vec2, Normal<Vec2>>,
{
    fn operator(
        &self,
        _: Normal<Vec3>,
        sdf: &Sdf,
        p: Vec3,
    ) -> <Normal<Vec3> as crate::prelude::Attribute>::Type {
        let d = sdf.evaluate(Normal::<Vec2>::default(), p.truncate());
        d.extend(1.0).normalize()
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Uv> for ExtrudeInteriorOp
where
    Uv: crate::prelude::Attribute,
    Sdf: crate::prelude::FieldFunction<Vec2, Uv>,
{
    fn operator(&self, attr: Uv, sdf: &Sdf, p: Vec3) -> <Uv as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p.truncate())
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

    use crate::{
        prelude::{BoundChecker, Circle, ExtrudeInterior, Point},
        test_op_attrs_2d, test_op_attrs_3d,
    };

    #[test]
    fn test_extrude_interior() {
        assert!(BoundChecker::<Vec3, ExtrudeInterior::<Circle>>::default().is_field());
    }

    test_op_attrs_3d!(ExtrudeInterior::<Point>);
}
