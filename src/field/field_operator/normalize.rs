use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, Field, Normal, Tangent, Uv, RaycastOutput},
};

use super::{FieldOperator, Operator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct NormalizeOp;

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Normal<Dim>> for NormalizeOp
where
    Sdf: Field<Dim, Normal<Dim>>,
    Dim: Clone + rust_gpu_bridge::Normalize,
{
    fn operator(&self, attr: Normal<Dim>, sdf: &Sdf, p: Dim) -> Dim {
        sdf.field(attr, p).clone().normalize()
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Tangent<Dim>> for NormalizeOp
where
    Sdf: Field<Dim, Tangent<Dim>>,
    Dim: Clone + rust_gpu_bridge::Normalize,
{
    fn operator(&self, attr: Tangent<Dim>, sdf: &Sdf, p: Dim) -> Dim {
        sdf.field(attr, p).clone().normalize()
    }
}

impl_passthrough_op_1!(NormalizeOp, Distance, Dim);
impl_passthrough_op_1!(NormalizeOp, Uv, Dim);
impl_passthrough_op_1!(NormalizeOp, Color, Dim);
impl_passthrough_op_1!(NormalizeOp, RaycastOutput, Dim);

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