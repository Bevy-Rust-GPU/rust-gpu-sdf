//! Override the colors of an SDF with the colors of another SDF

use rust_gpu_bridge::glam::Vec4;
use type_fields::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{Color, Distance, FieldOperator, Normal, Operator, Raycast, Tangent, Uv}, impl_passthrough_op_1,
};

/// Override the colors of an SDF with the colors of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct ProxyColorOp;

impl_passthrough_op_2!(ProxyColorOp, Distance<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyColorOp, Normal<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyColorOp, Tangent<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyColorOp, Uv<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyColorOp, Color<Dim>, 1, SdfB, Dim);
impl_passthrough_op_2!(ProxyColorOp, Raycast, 0, SdfA);

pub type ProxyColor<SdfA, SdfB> = Operator<ProxyColorOp, (SdfA, SdfB)>;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct WhiteOp;

impl_passthrough_op_1!(WhiteOp, Distance<Dim>, Dim);
impl_passthrough_op_1!(WhiteOp, Normal<Dim>, Dim);
impl_passthrough_op_1!(WhiteOp, Tangent<Dim>, Dim);
impl_passthrough_op_1!(WhiteOp, Uv<Dim>, Dim);

impl<Sdf, Input> crate::prelude::FieldOperator<Sdf, Color<Input>> for WhiteOp {
    fn operator(
        &self,
        _: &Sdf,
        _: &Input,
    ) -> <Color<Input> as crate::prelude::Attribute>::Output {
        Vec4::ONE
    }
}

impl_passthrough_op_2!(WhiteOp, Raycast, 0, SdfA);

pub type White<Sdf> = Operator<WhiteOp, Sdf>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{
        prelude::{Point, ProxyColor},
        test_op_attrs,
    };

    #[test]
    fn test_sdf_color() {
        ProxyColor::<Point, Point>::default();
    }

    test_op_attrs!(ProxyColor::<Point, Point>);
}
