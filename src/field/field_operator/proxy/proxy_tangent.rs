use type_fields::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{Color, Distance, FieldOperator, Normal, Operator, Tangent, Uv},
};

/// Override the tangents of an SDF with the tangents of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct ProxyTangentOp;

impl_passthrough_op_2!(ProxyTangentOp, Distance, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyTangentOp, Normal<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyTangentOp, Tangent<Dim>, 1, SdfB, Dim);
impl_passthrough_op_2!(ProxyTangentOp, Uv, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyTangentOp, Color, 0, SdfA, Dim);

pub type ProxyTangent<SdfA, SdfB> = Operator<ProxyTangentOp, (SdfA, SdfB)>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{
        prelude::{Point, ProxyTangent},
        test_op_attrs,
    };

    #[test]
    fn test_sdf_tangent() {
        ProxyTangent::<Point, Point>::default();
    }

    test_op_attrs!(ProxyTangent::<Point, Point>);
}
