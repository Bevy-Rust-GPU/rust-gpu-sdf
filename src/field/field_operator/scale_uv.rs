use rust_gpu_bridge::glam::Vec2;

use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, Field, Normal, RaycastOutput, Tangent, Uv},
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

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Uv> for ScaleUvOp
where
    Sdf: Field<Dim, Uv>,
{
    fn operator(&self, attr: Uv, sdf: &Sdf, p: Dim) -> <Uv as crate::prelude::Attribute>::Type {
        let uv = sdf.field(attr, p);
        uv * self.scale
    }
}

impl_passthrough_op_1!(ScaleUvOp, Distance, Dim);
impl_passthrough_op_1!(ScaleUvOp, Normal<Dim>, Dim);
impl_passthrough_op_1!(ScaleUvOp, Tangent<Dim>, Dim);
impl_passthrough_op_1!(ScaleUvOp, Color, Dim);
impl_passthrough_op_1!(ScaleUvOp, RaycastOutput, Dim);

pub type ScaleUv<Sdf> = Operator<ScaleUvOp, Sdf>;

