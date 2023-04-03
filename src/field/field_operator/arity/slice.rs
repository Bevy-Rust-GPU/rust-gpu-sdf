//! Take a 2D slice of a 3D field

use rust_gpu_bridge::glam::{Vec2, Vec3};
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv},
};

/// Take a 2D slice of a 3D field
#[derive(Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
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

impl<Sdf> FieldOperator<Sdf, Distance<Vec2>> for SliceOp
where
    Sdf: Field<Distance<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: &Vec2) -> f32 {
        let u = self.u * p.x;
        let v = self.v * p.y;
        sdf.field(&(u + v))
    }
}

impl<Sdf> FieldOperator<Sdf, Normal<Vec2>> for SliceOp
where
    Sdf: Field<Normal<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: &Vec2) -> Vec2 {
        let u = self.u * p.x;
        let v = self.v * p.y;
        let n = sdf.field(&(u + v));
        Vec2::new(n.dot(self.u), n.dot(self.v)).normalize()
    }
}

impl<Sdf> FieldOperator<Sdf, Tangent<Vec2>> for SliceOp
where
    Sdf: Field<Tangent<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: &Vec2) -> Vec2 {
        let u = self.u * p.x;
        let v = self.v * p.y;
        let n = sdf.field(&(u + v));
        Vec2::new(n.dot(self.u), n.dot(self.v)).normalize()
    }
}

impl<Sdf> FieldOperator<Sdf, Uv<Vec2>> for SliceOp
where
    Sdf: Field<Uv<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: &Vec2) -> Vec2 {
        let u = self.u * p.x;
        let v = self.v * p.y;
        sdf.field(&(u + v))
    }
}

impl_passthrough_op_1!(SliceOp, Color<Dim>, Dim);

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
