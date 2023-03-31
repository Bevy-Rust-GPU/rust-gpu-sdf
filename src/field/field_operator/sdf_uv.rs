//! Override the UVs of an SDF with the UVs of another SDF

use type_fields::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{Color, Distance, Normal, Tangent, Uv},
};

use super::{FieldOperator, Operator};

/// Override the UVs of an SDF with the UVs of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct SdfUvOp;

impl_passthrough_op_2!(SdfUvOp, Distance, 0, Dim);
impl_passthrough_op_2!(SdfUvOp, Normal<Dim>, 0, Dim);
impl_passthrough_op_2!(SdfUvOp, Tangent<Dim>, 0, Dim);
impl_passthrough_op_2!(SdfUvOp, Uv, 1, Dim);
impl_passthrough_op_2!(SdfUvOp, Color, 0, Dim);

pub type SdfUv<SdfA, SdfB> = Operator<SdfUvOp, (SdfA, SdfB)>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{prelude::Point, test_op_attrs};

    use super::SdfUv;

    #[test]
    fn test_sdf_uv() {
        SdfUv::<Point, Point>::default();
    }

    test_op_attrs!(SdfUv::<Point, Point>);
}
