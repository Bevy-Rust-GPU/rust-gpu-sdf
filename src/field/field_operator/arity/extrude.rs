//! Extrude a 2D distance field into 3D.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3, Vec3Swizzles},
    Abs, Sign,
};
use type_fields::Field;

use crate::prelude::{
    items::position::Position, AttrDistance, AttrNormal, AttrUv, Field, FieldOperator, Normal,
    Operator,
};

/// Extrude a 2D distance field into 3D.
#[derive(Default, Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct ExtrudeOp {
    pub axis: Vec3,
    pub depth: f32,
}

impl<Sdf> FieldOperator<Sdf, AttrDistance<Vec2>> for ExtrudeOp
where
    Sdf: Field<AttrDistance<f32>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        input: &Position<Vec2>,
    ) -> <AttrDistance<f32> as crate::prelude::Attribute>::Output {
        let d = *sdf.field(&input.x.into());
        let w = Vec2::new(d, input.y.abs() - self.depth);
        (w.x.max(w.y).min(0.0) + w.max(Vec2::ZERO).length()).into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrDistance<Vec3>> for ExtrudeOp
where
    Sdf: Field<AttrDistance<Vec2>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        input: &Position<Vec3>,
    ) -> <AttrDistance<Vec2> as crate::prelude::Attribute>::Output {
        let d = *sdf.field(&input.truncate().into());
        let w = Vec2::new(d, input.z.abs() - self.depth);
        (w.x.max(w.y).min(0.0) + w.max(Vec2::ZERO).length()).into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrNormal<Vec2>> for ExtrudeOp
where
    Sdf: Field<AttrNormal<f32>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec2>) -> Normal<Vec2> {
        let d = *sdf.field(&p.x.into());
        let w = Vec2::new(d, p.y.abs() - self.depth);
        let s = p.y.sign();

        let g = w.x.max(w.y);
        let q = w.max(Vec2::ZERO);
        let l = q.length();

        let m = s
            * (if g > 0.0 {
                q / l
            } else {
                if w.x > w.y {
                    Vec2::X
                } else {
                    Vec2::Y
                }
            });

        m.into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrNormal<Vec3>> for ExtrudeOp
where
    Sdf: Field<AttrNormal<Vec2>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec3>) -> Normal<Vec3> {
        let d = sdf.field(&p.xy().into());
        if p.z.abs() > p.xy().length() * 0.5 {
            Vec3::new(0.0, 0.0, p.z.sign())
        } else {
            d.extend(0.0)
        }
        .normalize()
        .into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrUv<Vec2>> for ExtrudeOp
where
    Sdf: crate::prelude::Field<AttrUv<f32>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: &Position<Vec2>,
    ) -> <AttrUv<Vec2> as crate::prelude::Attribute>::Output {
        (*sdf.field(&p.x.into()) + Vec2::new(0.0, p.y.abs())).into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrUv<Vec3>> for ExtrudeOp
where
    Sdf: crate::prelude::Field<AttrUv<Vec2>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: &Position<Vec3>,
    ) -> <AttrUv<Vec3> as crate::prelude::Attribute>::Output {
        (*sdf.field(&p.truncate().into()) + Vec2::new(0.0, p.z.abs())).into()
    }
}

/// Extrude a 2D distance field into 3D.
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
    use crate::{
        prelude::{BoundTester, Circle, Extrude, Point, Sphere},
        test_op_attrs_2d, test_op_attrs_3d,
    };

    #[test]
    fn test_extrude_2d() {
        assert!(BoundTester::<Extrude::<Circle>>::default().is_field_2d());
    }

    #[test]
    fn test_extrude_3d() {
        assert!(BoundTester::<Extrude::<Sphere>>::default().is_field_3d());
    }

    test_op_attrs_2d!(Extrude::<Point>);
    test_op_attrs_3d!(Extrude::<Point>);
}
