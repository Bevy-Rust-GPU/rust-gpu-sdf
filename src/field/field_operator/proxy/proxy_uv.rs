//! Override the UVs of an SDF with the UVs of another SDF

use type_fields::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{Color, Distance, FieldOperator, Normal, Operator, Tangent, Uv},
};

/// Override the UVs of an SDF with the UVs of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct ProxyUvOp;

impl_passthrough_op_2!(ProxyUvOp, Distance, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyUvOp, Normal<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyUvOp, Tangent<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyUvOp, Uv, 1, SdfB, Dim);
impl_passthrough_op_2!(ProxyUvOp, Color, 0, SdfA, Dim);

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
