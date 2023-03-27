use crate::{
    impl_passthrough_op_2,
    prelude::{Color, Distance, Normal, Tangent, Uv},
};

use super::{FieldOperator, Operator};

/// Override the normals of an SDF with the normals of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SdfNormalOp;

impl_passthrough_op_2!(SdfNormalOp, <Dim>, Distance, 0);
impl_passthrough_op_2!(SdfNormalOp, <Dim>, Normal<Dim>, 1);
impl_passthrough_op_2!(SdfNormalOp, <Dim>, Tangent<Dim>, 0);
impl_passthrough_op_2!(SdfNormalOp, <Dim>, Uv, 0);
impl_passthrough_op_2!(SdfNormalOp, <Dim>, Color, 0);

pub type SdfNormal<SdfA, SdfB> = Operator<SdfNormalOp, (SdfA, SdfB)>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{prelude::Point, test_op_attrs};

    use super::SdfNormal;

    #[test]
    fn test_sdf_normal() {
        SdfNormal::<Point, Point>::default();
    }

    test_op_attrs!(SdfNormal::<Point, Point>);
}
