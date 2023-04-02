use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, Field, Normal, RaycastOutput, Tangent, Uv},
};
use rust_gpu_bridge::{glam::Vec3, Mod};

use super::{FieldOperator, Operator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
pub struct CheckerOp;

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Color> for CheckerOp
where
    Sdf: Field<Dim, Uv>,
    Dim: Mod,
{
    fn operator(
        &self,
        _: Color,
        sdf: &Sdf,
        p: Dim,
    ) -> <Color as crate::prelude::Attribute>::Type {
        let uv = sdf.field(Uv, p);
        let checker = uv.round();
        let checker = (checker.x + checker.y).modulo(2.0) / 2.0;
        Vec3::splat(checker).extend(1.0)
    }
}

impl_passthrough_op_1!(CheckerOp, Distance, Dim);
impl_passthrough_op_1!(CheckerOp, Normal<Dim>, Dim);
impl_passthrough_op_1!(CheckerOp, Tangent<Dim>, Dim);
impl_passthrough_op_1!(CheckerOp, Uv, Dim);
impl_passthrough_op_1!(CheckerOp, RaycastOutput, Dim);

pub type Checker<Sdf> = Operator<CheckerOp, Sdf>;

