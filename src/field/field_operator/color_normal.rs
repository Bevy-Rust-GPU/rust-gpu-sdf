use core::ops::{Add, Mul};

use rust_gpu_bridge::{glam::Vec4, Splat, ToVec};

use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, Field, Normal, Tangent, Uv, Raycast},
};

use super::{FieldOperator, Operator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
pub struct ColorNormalOp;

impl<Sdf, Input> FieldOperator<Sdf, Color<Input>> for ColorNormalOp
where
    Sdf: Field<Normal<Input>>,
    Input: Add<Input, Output = Input> + Mul<Input, Output = Input> + Splat + ToVec<Vec4>,
{
    fn operator(&self, sdf: &Sdf, p: &Input) -> Vec4 {
        let normal = sdf.field(p);
        let normal = normal * Input::splat(0.5) + Input::splat(0.5);
        let mut color = normal.to_vec();
        color.w = 1.0;
        color
    }
}

impl_passthrough_op_1!(ColorNormalOp, Distance<Dim>, Dim);
impl_passthrough_op_1!(ColorNormalOp, Normal<Dim>, Dim);
impl_passthrough_op_1!(ColorNormalOp, Tangent<Dim>, Dim);
impl_passthrough_op_1!(ColorNormalOp, Uv<Dim>, Dim);
impl_passthrough_op_1!(ColorNormalOp, Raycast,);

pub type ColorNormal<Sdf> = Operator<ColorNormalOp, Sdf>;
