use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, Field, Normal, Raycast, Tangent, Uv},
};
use rust_gpu_bridge::{glam::Vec3, Mod};

use super::{FieldOperator, Operator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
pub struct CheckerOp;

impl<Sdf, Dim> FieldOperator<Sdf, Color<Dim>> for CheckerOp
where
    Sdf: Field<Uv<Dim>>,
    Dim: Mod,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> <Color<Dim> as crate::prelude::Attribute>::Output {
        let uv = sdf.field(p);
        let checker = uv.round();
        let checker = (checker.x + checker.y).modulo(2.0) / 2.0;
        Vec3::splat(checker).extend(1.0)
    }
}

impl_passthrough_op_1!(CheckerOp, Distance<Dim>, Dim);
impl_passthrough_op_1!(CheckerOp, Normal<Dim>, Dim);
impl_passthrough_op_1!(CheckerOp, Tangent<Dim>, Dim);
impl_passthrough_op_1!(CheckerOp, Uv<Dim>, Dim);
impl_passthrough_op_1!(CheckerOp, Raycast,);

pub type Checker<Sdf> = Operator<CheckerOp, Sdf>;
