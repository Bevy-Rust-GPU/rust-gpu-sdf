use core::ops::{Add, Mul};

use rust_gpu_bridge::{glam::Vec4, Splat, ToVec};

use crate::{
    impl_passthrough_op_1,
    prelude::{AttrColor, AttrDistance, Field, AttrNormal, Raycast, AttrTangent, AttrUv, items::position::Position, Color},
};

use super::{FieldOperator, Operator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
pub struct ColorUvOp;

impl<Sdf, Input> FieldOperator<Sdf, AttrColor<Input>> for ColorUvOp
where
    Sdf: Field<AttrUv<Input>>,
    Input: Add<Input, Output = Input> + Mul<Input, Output = Input> + Splat + ToVec<Vec4>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Input>) -> Color {
        let uv = sdf.field(p);
        uv.extend(0.0).extend(1.0).into()
    }
}

impl_passthrough_op_1!(ColorUvOp, AttrDistance<Dim>, Dim);
impl_passthrough_op_1!(ColorUvOp, AttrNormal<Dim>, Dim);
impl_passthrough_op_1!(ColorUvOp, AttrTangent<Dim>, Dim);
impl_passthrough_op_1!(ColorUvOp, AttrUv<Dim>, Dim);
impl_passthrough_op_1!(ColorUvOp, Raycast,);

pub type ColorUv<Sdf> = Operator<ColorUvOp, Sdf>;
