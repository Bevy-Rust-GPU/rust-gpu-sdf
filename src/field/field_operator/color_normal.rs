use core::ops::{Add, Mul};

use rust_gpu_bridge::{glam::Vec4, Splat, ToVec};

use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, Field, Normal, RaycastOutput, Tangent, Uv},
};

use super::{FieldOperator, Operator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
pub struct ColorNormalOp;

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Color> for ColorNormalOp
where
    Sdf: Field<Dim, Normal<Dim>>,
    Dim: Add<Dim, Output = Dim> + Mul<Dim, Output = Dim> + Splat + ToVec<Vec4>,
{
    fn operator(&self, _: Color, sdf: &Sdf, p: Dim) -> Vec4 {
        let normal = sdf.field(Normal::<Dim>::default(), p);
        let normal = normal * Dim::splat(0.5) + Dim::splat(0.5);
        let mut color = normal.to_vec();
        color.w = 1.0;
        color
    }
}

impl_passthrough_op_1!(ColorNormalOp, Distance, Dim);
impl_passthrough_op_1!(ColorNormalOp, Normal<Dim>, Dim);
impl_passthrough_op_1!(ColorNormalOp, Tangent<Dim>, Dim);
impl_passthrough_op_1!(ColorNormalOp, Uv, Dim);
impl_passthrough_op_1!(ColorNormalOp, RaycastOutput, Dim);

pub type ColorNormal<Sdf> = Operator<ColorNormalOp, Sdf>;

