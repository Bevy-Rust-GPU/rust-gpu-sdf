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

impl<Sdf> FieldOperator<Sdf, Vec2, Distance> for ExtrudeOp
where
    Sdf: Field<f32, Distance>,
{
    fn operator(
        &self,
        attr: Distance,
        sdf: &Sdf,
        p: Vec2,
    ) -> <Distance as crate::prelude::Attribute>::Type {
        let d = sdf.field(attr, p.x);
        let w = Vec2::new(d, p.y.abs() - self.depth);
        w.x.max(w.y).min(0.0) + w.max(Vec2::ZERO).length()
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Distance> for ExtrudeOp
where
    Sdf: Field<Vec2, Distance>,
{
    fn operator(
        &self,
        attr: Distance,
        sdf: &Sdf,
        p: Vec3,
    ) -> <Distance as crate::prelude::Attribute>::Type {
        let d = sdf.field(attr, p.truncate());
        let w = Vec2::new(d, p.z.abs() - self.depth);
        w.x.max(w.y).min(0.0) + w.max(Vec2::ZERO).length()
    }
}

impl<Sdf> FieldOperator<Sdf, Vec2, Normal<Vec2>> for ExtrudeOp
where
    Sdf: Field<f32, Normal<f32>>,
{
    fn operator(&self, _: Normal<Vec2>, sdf: &Sdf, p: Vec2) -> Vec2 {
        let d = sdf.field(Normal::<f32>::default(), p.x);
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

impl<Sdf> FieldOperator<Sdf, Vec3, Normal<Vec3>> for ExtrudeOp
where
    Sdf: Field<Vec2, Normal<Vec2>>,
{
    fn operator(&self, _: Normal<Vec3>, sdf: &Sdf, p: Vec3) -> Vec3 {
        let d = sdf.field(Normal::<Vec2>::default(), p.xy());
        if p.z.abs() > p.xy().length() * 0.5 {
            Vec3::new(0.0, 0.0, p.z.sign())
        } else {
            d.extend(0.0)
        }
        .normalize()
    }
}

impl<Sdf> FieldOperator<Sdf, Vec2, Uv> for ExtrudeOp
where
    Uv: crate::prelude::Attribute,
    Sdf: crate::prelude::Field<f32, Uv>,
{
    fn operator(&self, attr: Uv, sdf: &Sdf, p: Vec2) -> <Uv as crate::prelude::Attribute>::Type {
        sdf.field(attr, p.x) + Vec2::new(0.0, p.y.abs())
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Uv> for ExtrudeOp
where
    Uv: crate::prelude::Attribute,
    Sdf: crate::prelude::Field<Vec2, Uv>,
{
    fn operator(&self, attr: Uv, sdf: &Sdf, p: Vec3) -> <Uv as crate::prelude::Attribute>::Type {
        sdf.field(attr, p.truncate()) + Vec2::new(0.0, p.z.abs())
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
    use rust_gpu_bridge::glam::{Vec2, Vec3};

    use crate::{
        prelude::{BoundTester, Circle, Extrude, Point, Sphere},
        test_op_attrs_2d, test_op_attrs_3d,
    };

    #[test]
    fn test_extrude_2d() {
        assert!(BoundTester::<Vec2, Extrude::<Circle>>::default().is_field());
    }

    #[test]
    fn test_extrude_3d() {
        assert!(BoundTester::<Vec3, Extrude::<Sphere>>::default().is_field());
    }

    test_op_attrs_2d!(Extrude::<Point>);
    test_op_attrs_3d!(Extrude::<Point>);
}
