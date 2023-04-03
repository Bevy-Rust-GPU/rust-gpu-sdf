use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, Field, Normal, Tangent, Uv, Raycast},
};

use super::{FieldOperator, Operator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct NormalizeOp;

impl<Sdf, Input> FieldOperator<Sdf, Normal<Input>> for NormalizeOp
where
    Sdf: Field<Normal<Input>>,
    Input: Clone + rust_gpu_bridge::Normalize,
{
    fn operator(&self, sdf: &Sdf, p: &Input) -> Input {
        sdf.field(p).clone().normalize()
    }
}

impl<Sdf, Input> FieldOperator<Sdf, Tangent<Input>> for NormalizeOp
where
    Sdf: Field<Tangent<Input>>,
    Input: Clone + rust_gpu_bridge::Normalize,
{
    fn operator(&self, sdf: &Sdf, p: &Input) -> Input {
        sdf.field(p).clone().normalize()
    }
}

impl_passthrough_op_1!(NormalizeOp, Distance<Dim>, Dim);
impl_passthrough_op_1!(NormalizeOp, Uv<Dim>, Dim);
impl_passthrough_op_1!(NormalizeOp, Color<Dim>, Dim);
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
