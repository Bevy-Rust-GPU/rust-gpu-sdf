use type_fields::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{Color, Distance, Normal, Tangent, Uv},
};

use super::{FieldOperator, Operator};

/// Override the tangents of an SDF with the tangents of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct SdfTangentOp;

impl_passthrough_op_2!(SdfTangentOp, Distance, 0, Dim);
impl_passthrough_op_2!(SdfTangentOp, Normal<Dim>, 0, Dim);
impl_passthrough_op_2!(SdfTangentOp, Tangent<Dim>, 1, Dim);
impl_passthrough_op_2!(SdfTangentOp, Uv, 0, Dim);
impl_passthrough_op_2!(SdfTangentOp, Color, 0, Dim);

pub type SdfTangent<SdfA, SdfB> = Operator<SdfTangentOp, (SdfA, SdfB)>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{prelude::Point, test_op_attrs};

    use super::SdfTangent;

    #[test]
    fn test_sdf_tangent() {
        SdfTangent::<Point, Point>::default();
    }

    test_op_attrs!(SdfTangent::<Point, Point>);
}
