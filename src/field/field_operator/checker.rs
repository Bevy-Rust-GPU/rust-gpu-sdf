use crate::{
    impl_passthrough_op_1,
    prelude::{AttrColor, AttrDistance, Field, AttrNormal, Raycast, AttrTangent, AttrUv, items::position::Position},
};
use rust_gpu_bridge::{glam::Vec3, Mod};

use super::{FieldOperator, Operator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
pub struct CheckerOp;

impl<Sdf, Input> FieldOperator<Sdf, AttrColor<Input>> for CheckerOp
where
    Sdf: Field<AttrUv<Input>>,
    Input: Mod,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Input>) -> <AttrColor<Input> as crate::prelude::Attribute>::Output {
        let uv = sdf.field(p);
        let checker = uv.round();
        let checker = (checker.x + checker.y).modulo(2.0) / 2.0;
        Vec3::splat(checker).extend(1.0).into()
    }
}

impl_passthrough_op_1!(CheckerOp, AttrDistance<Dim>, Dim);
impl_passthrough_op_1!(CheckerOp, AttrNormal<Dim>, Dim);
impl_passthrough_op_1!(CheckerOp, AttrTangent<Dim>, Dim);
impl_passthrough_op_1!(CheckerOp, AttrUv<Dim>, Dim);
impl_passthrough_op_1!(CheckerOp, Raycast,);

pub type Checker<Sdf> = Operator<CheckerOp, Sdf>;
