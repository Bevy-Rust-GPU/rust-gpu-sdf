use rust_gpu_bridge::glam::{Vec3, Vec4};
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, Normal, Tangent, Uv},
};

use super::{FieldOperator, Operator};

/// Apply triplanar UV mapping to the provided SDF
#[derive(Copy, Clone, PartialEq, Field)]
pub struct ColorizeOp {
    pub color: Vec4,
}

impl Default for ColorizeOp {
    fn default() -> Self {
        ColorizeOp { color: Vec4::ONE }
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Color> for ColorizeOp {
    fn operator(&self, _: Color, _: &Sdf, _: Vec3) -> <Color as crate::prelude::Attribute>::Type {
        self.color
    }
}

impl_passthrough_op_1!(ColorizeOp, <Dim>, Distance);
impl_passthrough_op_1!(ColorizeOp, <Dim>, Normal<Dim>);
impl_passthrough_op_1!(ColorizeOp, <Dim>, Tangent<Dim>);
impl_passthrough_op_1!(ColorizeOp, <Dim>, Uv);

pub type Colorize<Sdf> = Operator<ColorizeOp, Sdf>;

impl<Sdf> Colorize<Sdf> {
    pub fn color(&mut self) -> &mut Vec4 {
        self.op().color()
    }
}
