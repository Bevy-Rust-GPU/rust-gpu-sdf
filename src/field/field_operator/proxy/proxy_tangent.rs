use type_fields::macros::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{AttrColor, AttrDistance, FieldOperator, AttrNormal, Operator, Raycast, AttrTangent, AttrUv},
};

/// Override the tangents of an SDF with the tangents of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct ProxyTangentOp;

impl_passthrough_op_2!(ProxyTangentOp, AttrDistance<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyTangentOp, AttrNormal<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyTangentOp, AttrTangent<Dim>, 1, SdfB, Dim);
impl_passthrough_op_2!(ProxyTangentOp, AttrUv<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyTangentOp, AttrColor<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyTangentOp, Raycast, 0, SdfA);

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
