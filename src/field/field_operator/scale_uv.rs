use rust_gpu_bridge::glam::Vec2;

use crate::{
    impl_passthrough_op_1,
    prelude::{AttrColor, AttrDistance, Field, AttrNormal, Raycast, AttrTangent, AttrUv, items::position::Position},
};

use super::{FieldOperator, Operator};

#[derive(Copy, Clone, PartialEq, type_fields::Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
pub struct ScaleUvOp {
    pub scale: Vec2,
}

impl Default for ScaleUvOp {
    fn default() -> Self {
        ScaleUvOp {
            scale: Vec2::ONE * 10.0,
        }
    }
}

impl<Sdf, Input> FieldOperator<Sdf, AttrUv<Input>> for ScaleUvOp
where
    Sdf: Field<AttrUv<Input>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Input>) -> <AttrUv<Input> as crate::prelude::Attribute>::Output {
        let uv = *sdf.field(p);
        (uv * self.scale).into()
    }
}

impl_passthrough_op_1!(ScaleUvOp, AttrDistance<Dim>, Dim);
impl_passthrough_op_1!(ScaleUvOp, AttrNormal<Dim>, Dim);
impl_passthrough_op_1!(ScaleUvOp, AttrTangent<Dim>, Dim);
impl_passthrough_op_1!(ScaleUvOp, AttrColor<Dim>, Dim);
impl_passthrough_op_1!(ScaleUvOp, Raycast,);

pub type ScaleUv<Sdf> = Operator<ScaleUvOp, Sdf>;
