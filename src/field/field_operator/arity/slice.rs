//! Take a 2D slice of a 3D field

use rust_gpu_bridge::glam::{Vec2, Vec3};
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv},
};

/// Take a 2D slice of a 3D field
#[derive(Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct SliceOp {
    pub u: Vec3,
    pub v: Vec3,
}

impl Default for SliceOp {
    fn default() -> Self {
        SliceOp {
            u: Vec3::X,
            v: Vec3::Y,
        }
    }
}

impl<Sdf> FieldOperator<Sdf, Vec2, Distance> for SliceOp
where
    Sdf: Field<Vec3, Distance>,
{
    fn operator(&self, attr: Distance, sdf: &Sdf, p: Vec2) -> f32 {
        let u = self.u * p.x;
        let v = self.v * p.y;
        sdf.field(attr, u + v)
    }
}

impl<Sdf> FieldOperator<Sdf, Vec2, Normal<Vec2>> for SliceOp
where
    Sdf: Field<Vec3, Normal<Vec3>>,
{
    fn operator(&self, _: Normal<Vec2>, sdf: &Sdf, p: Vec2) -> Vec2 {
        let u = self.u * p.x;
        let v = self.v * p.y;
        let n = sdf.field(Normal::<Vec3>::default(), u + v);
        Vec2::new(n.dot(self.u), n.dot(self.v)).normalize()
    }
}

impl<Sdf> FieldOperator<Sdf, Vec2, Tangent<Vec2>> for SliceOp
where
    Sdf: Field<Vec3, Tangent<Vec3>>,
{
    fn operator(&self, _: Tangent<Vec2>, sdf: &Sdf, p: Vec2) -> Vec2 {
        let u = self.u * p.x;
        let v = self.v * p.y;
        let n = sdf.field(Tangent::<Vec3>::default(), u + v);
        Vec2::new(n.dot(self.u), n.dot(self.v)).normalize()
    }
}

impl<Sdf> FieldOperator<Sdf, Vec2, Uv> for SliceOp
where
    Sdf: Field<Vec3, Uv>,
{
    fn operator(&self, attr: Uv, sdf: &Sdf, p: Vec2) -> Vec2 {
        let u = self.u * p.x;
        let v = self.v * p.y;
        sdf.field(attr, u + v)
    }
}

impl_passthrough_op_1!(SliceOp, Color, Dim);

/// Take a 2D slice of a 3D field
pub type Slice<Sdf> = Operator<SliceOp, Sdf>;

impl<Sdf> Slice<Sdf> {
    pub fn u(&mut self) -> &mut Vec3 {
        &mut self.op().u
    }

    pub fn v(&mut self) -> &mut Vec3 {
        &mut self.op().v
    }
}
