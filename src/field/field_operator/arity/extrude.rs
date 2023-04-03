//! Extrude a 2D distance field into 3D.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3, Vec3Swizzles},
    Abs, Sign,
};
use type_fields::Field;

use crate::prelude::{Distance, Field, FieldOperator, Normal, Operator, Uv};

/// Extrude a 2D distance field into 3D.
#[derive(Default, Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct ExtrudeOp {
    pub axis: Vec3,
    pub depth: f32,
}

impl<Sdf> FieldOperator<Sdf, Distance<Vec2>> for ExtrudeOp
where
    Sdf: Field<Distance<f32>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: Vec2,
    ) -> <Distance<f32> as crate::prelude::Attribute>::Output {
        let d = sdf.field(p.x);
        let w = Vec2::new(d, p.y.abs() - self.depth);
        w.x.max(w.y).min(0.0) + w.max(Vec2::ZERO).length()
    }
}

impl<Sdf> FieldOperator<Sdf, Distance<Vec3>> for ExtrudeOp
where
    Sdf: Field<Distance<Vec2>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: Vec3,
    ) -> <Distance<Vec2> as crate::prelude::Attribute>::Output {
        let d = sdf.field(p.truncate());
        let w = Vec2::new(d, p.z.abs() - self.depth);
        w.x.max(w.y).min(0.0) + w.max(Vec2::ZERO).length()
    }
}

impl<Sdf> FieldOperator<Sdf, Normal<Vec2>> for ExtrudeOp
where
    Sdf: Field<Normal<f32>>,
{
    fn operator(&self, sdf: &Sdf, p: Vec2) -> Vec2 {
        let d = sdf.field(p.x);
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

        m
    }
}

impl<Sdf> FieldOperator<Sdf, Normal<Vec3>> for ExtrudeOp
where
    Sdf: Field<Normal<Vec2>>,
{
    fn operator(&self, sdf: &Sdf, p: Vec3) -> Vec3 {
        let d = sdf.field(p.xy());
        if p.z.abs() > p.xy().length() * 0.5 {
            Vec3::new(0.0, 0.0, p.z.sign())
        } else {
            d.extend(0.0)
        }
        .normalize()
    }
}

impl<Sdf> FieldOperator<Sdf, Uv<Vec2>> for ExtrudeOp
where
    Sdf: crate::prelude::Field<Uv<f32>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: Vec2,
    ) -> <Uv<Vec2> as crate::prelude::Attribute>::Output {
        sdf.field(p.x) + Vec2::new(0.0, p.y.abs())
    }
}

impl<Sdf> FieldOperator<Sdf, Uv<Vec3>> for ExtrudeOp
where
    Sdf: crate::prelude::Field<Uv<Vec2>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: Vec3,
    ) -> <Uv<Vec3> as crate::prelude::Attribute>::Output {
        sdf.field(p.truncate()) + Vec2::new(0.0, p.z.abs())
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
