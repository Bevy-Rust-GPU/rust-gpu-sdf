use core::ops::{Add, Mul};

use rust_gpu_bridge::{glam::Vec4, Splat, ToVec};

use crate::{
    impl_passthrough_op_1,
    prelude::{AttrColor, AttrDistance, Field, AttrNormal, Raycast, AttrTangent, AttrUv, items::position::Position, Color},
};

use super::{FieldOperator, Operator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
pub struct ColorTangentOp;

impl<Sdf, Input> FieldOperator<Sdf, AttrColor<Input>> for ColorTangentOp
where
    Sdf: Field<AttrTangent<Input>>,
    Input: Clone + Add<Input, Output = Input> + Mul<Input, Output = Input> + Splat + ToVec<Vec4>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Input>) -> Color {
        let tangent = (*sdf.field(p)).clone();
        let tangent = tangent * Input::splat(0.5) + Input::splat(0.5);
        let mut color = tangent.to_vec();
        color.w = 1.0;
        color.into()
    }
}

impl_passthrough_op_1!(ColorTangentOp, AttrDistance<Dim>, Dim);
impl_passthrough_op_1!(ColorTangentOp, AttrNormal<Dim>, Dim);
impl_passthrough_op_1!(ColorTangentOp, AttrTangent<Dim>, Dim);
impl_passthrough_op_1!(ColorTangentOp, AttrUv<Dim>, Dim);
impl_passthrough_op_1!(ColorTangentOp, Raycast,);

pub type ColorTangent<Sdf> = Operator<ColorTangentOp, Sdf>;

