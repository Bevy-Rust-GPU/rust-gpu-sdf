use rust_gpu_bridge::glam::Vec2;

use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, Field, Normal, Raycast, Tangent, Uv},
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

impl<Sdf, Dim> FieldOperator<Sdf, Uv<Dim>> for ScaleUvOp
where
    Sdf: Field<Uv<Dim>>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> <Uv<Dim> as crate::prelude::Attribute>::Output {
        let uv = sdf.field(p);
        uv * self.scale
    }
}

impl_passthrough_op_1!(ScaleUvOp, Distance<Dim>, Dim);
impl_passthrough_op_1!(ScaleUvOp, Normal<Dim>, Dim);
impl_passthrough_op_1!(ScaleUvOp, Tangent<Dim>, Dim);
impl_passthrough_op_1!(ScaleUvOp, Color<Dim>, Dim);
impl_passthrough_op_1!(ScaleUvOp, Raycast,);

pub type ScaleUv<Sdf> = Operator<ScaleUvOp, Sdf>;
