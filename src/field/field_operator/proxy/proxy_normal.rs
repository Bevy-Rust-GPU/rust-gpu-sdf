use crate::{
    impl_passthrough_op_2,
    prelude::{Color, Distance, FieldOperator, Normal, Operator, Tangent, Uv, RaycastOutput},
};

/// Override the normals of an SDF with the normals of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct ProxyNormalOp;

impl_passthrough_op_2!(ProxyNormalOp, Distance, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyNormalOp, Normal<Dim>, 1, SdfB, Dim);
impl_passthrough_op_2!(ProxyNormalOp, Tangent<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyNormalOp, Uv, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyNormalOp, Color, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyNormalOp, RaycastOutput, 0, SdfA, Dim);

pub type ProxyNormal<SdfA, SdfB> = Operator<ProxyNormalOp, (SdfA, SdfB)>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{prelude::{Point, ProxyNormal}, test_op_attrs};

    #[test]
    fn test_sdf_normal() {
        ProxyNormal::<Point, Point>::default();
    }

    test_op_attrs!(ProxyNormal::<Point, Point>);
}