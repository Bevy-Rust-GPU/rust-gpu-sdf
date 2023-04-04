use crate::{
    impl_passthrough_op_1,
    prelude::{
        items::position::Position, AttrColor, AttrDistance, AttrNormal, AttrTangent, AttrUv, Field,
        Normal, Raycast, Tangent,
    },
};

use super::{FieldOperator, Operator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct NormalizeOp;

impl<Sdf, Input> FieldOperator<Sdf, AttrNormal<Input>> for NormalizeOp
where
    Sdf: Field<AttrNormal<Input>>,
    Input: Clone + rust_gpu_bridge::Normalize,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Input>) -> Normal<Input> {
        (*sdf.field(p)).clone().normalize().into()
    }
}

impl<Sdf, Input> FieldOperator<Sdf, AttrTangent<Input>> for NormalizeOp
where
    Sdf: Field<AttrTangent<Input>>,
    Input: Clone + rust_gpu_bridge::Normalize,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Input>) -> Tangent<Input> {
        (*sdf.field(p)).clone().normalize().into()
    }
}

impl_passthrough_op_1!(NormalizeOp, AttrDistance<Dim>, Dim);
impl_passthrough_op_1!(NormalizeOp, AttrUv<Dim>, Dim);
impl_passthrough_op_1!(NormalizeOp, AttrColor<Dim>, Dim);
impl_passthrough_op_1!(NormalizeOp, Raycast,);

pub type Normalize<Sdf> = Operator<NormalizeOp, Sdf>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{prelude::Point, test_op_attrs};

    use super::Normalize;

    #[test]
    fn test_normalize() {
        Normalize::<Point>::default();
    }

    test_op_attrs!(Normalize::<Point>);
}
