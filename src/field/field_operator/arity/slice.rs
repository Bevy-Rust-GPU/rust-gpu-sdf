//! Take a 2D slice of a 3D field

use rust_gpu_bridge::glam::{Vec2, Vec3};
use type_fields::macros::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{
        items::position::Position, AttrColor, AttrDistance, AttrNormal, AttrTangent, AttrUv,
        Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv,
    },
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

impl<Sdf> FieldOperator<Sdf, AttrDistance<Vec2>> for SliceOp
where
    Sdf: Field<AttrDistance<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec2>) -> Distance {
        let u = self.u * p.x;
        let v = self.v * p.y;
        sdf.field(&(u + v).into()).into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrNormal<Vec2>> for SliceOp
where
    Sdf: Field<AttrNormal<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec2>) -> Normal<Vec2> {
        let u = self.u * p.x;
        let v = self.v * p.y;
        let n = sdf.field(&(u + v).into());
        Vec2::new(n.dot(self.u), n.dot(self.v)).normalize().into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrTangent<Vec2>> for SliceOp
where
    Sdf: Field<AttrTangent<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec2>) -> Tangent<Vec2> {
        let u = self.u * p.x;
        let v = self.v * p.y;
        let n = sdf.field(&(u + v).into());
        Vec2::new(n.dot(self.u), n.dot(self.v)).normalize().into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrUv<Vec2>> for SliceOp
where
    Sdf: Field<AttrUv<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec2>) -> Uv {
        let u = self.u * p.x;
        let v = self.v * p.y;
        sdf.field(&(u + v).into())
    }
}

impl_passthrough_op_1!(SliceOp, AttrColor<Dim>, Dim);

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
