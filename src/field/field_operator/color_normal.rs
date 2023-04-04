use core::ops::{Add, Mul};

use rust_gpu_bridge::{glam::Vec4, Splat, ToVec};

use crate::{
    impl_passthrough_op_1,
    prelude::{AttrColor, AttrDistance, Field, AttrNormal, AttrTangent, AttrUv, Raycast, items::position::Position, Color},
};

use super::{FieldOperator, Operator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
pub struct ColorNormalOp;

impl<Sdf, Input> FieldOperator<Sdf, AttrColor<Input>> for ColorNormalOp
where
    Sdf: Field<AttrNormal<Input>>,
    Input: Clone + Add<Input, Output = Input> + Mul<Input, Output = Input> + Splat + ToVec<Vec4>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Input>) -> Color {
        let normal = (*sdf.field(p)).clone();
        let normal = normal * Input::splat(0.5) + Input::splat(0.5);
        let mut color = normal.to_vec();
        color.w = 1.0;
        color.into()
    }
}

impl_passthrough_op_1!(ColorNormalOp, AttrDistance<Dim>, Dim);
impl_passthrough_op_1!(ColorNormalOp, AttrNormal<Dim>, Dim);
impl_passthrough_op_1!(ColorNormalOp, AttrTangent<Dim>, Dim);
impl_passthrough_op_1!(ColorNormalOp, AttrUv<Dim>, Dim);
impl_passthrough_op_1!(ColorNormalOp, Raycast,);

pub type ColorNormal<Sdf> = Operator<ColorNormalOp, Sdf>;
