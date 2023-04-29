//! Override the UVs of an SDF with the UVs of another SDF

use type_fields::macros::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{AttrColor, AttrDistance, FieldOperator, AttrNormal, Operator, Raycast, AttrTangent, AttrUv},
};

/// Override the UVs of an SDF with the UVs of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct ProxyUvOp;

impl_passthrough_op_2!(ProxyUvOp, AttrDistance<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyUvOp, AttrNormal<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyUvOp, AttrTangent<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyUvOp, AttrUv<Dim>, 1, SdfB, Dim);
impl_passthrough_op_2!(ProxyUvOp, AttrColor<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyUvOp, Raycast, 0, SdfA);

pub type ProxyUv<SdfA, SdfB> = Operator<ProxyUvOp, (SdfA, SdfB)>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{
        prelude::{Point, ProxyUv},
        test_op_attrs,
    };

    #[test]
    fn test_sdf_uv() {
        ProxyUv::<Point, Point>::default();
    }

    test_op_attrs!(ProxyUv::<Point, Point>);
}
