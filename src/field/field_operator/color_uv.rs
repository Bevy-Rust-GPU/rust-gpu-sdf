use core::ops::{Add, Mul};

use rust_gpu_bridge::{glam::Vec4, Splat, ToVec};

use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, Field, Normal, Raycast, Tangent, Uv},
};

use super::{FieldOperator, Operator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
pub struct ColorUvOp;

impl<Sdf, Dim> FieldOperator<Sdf, Color<Dim>> for ColorUvOp
where
    Sdf: Field<Uv<Dim>>,
    Dim: Add<Dim, Output = Dim> + Mul<Dim, Output = Dim> + Splat + ToVec<Vec4>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Vec4 {
        let uv = sdf.field(p);
        uv.extend(0.0).extend(1.0)
    }
}

impl_passthrough_op_1!(ColorUvOp, Distance<Dim>, Dim);
impl_passthrough_op_1!(ColorUvOp, Normal<Dim>, Dim);
impl_passthrough_op_1!(ColorUvOp, Tangent<Dim>, Dim);
impl_passthrough_op_1!(ColorUvOp, Uv<Dim>, Dim);
impl_passthrough_op_1!(ColorUvOp, Raycast,);

pub type ColorUv<Sdf> = Operator<ColorUvOp, Sdf>;
