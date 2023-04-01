//! Override the colors of an SDF with the colors of another SDF

use type_fields::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{Color, Distance, FieldOperator, Normal, Operator, Tangent, Uv},
};

/// Override the colors of an SDF with the colors of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct ProxyColorOp;

impl_passthrough_op_2!(ProxyColorOp, Distance, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyColorOp, Normal<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyColorOp, Tangent<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyColorOp, Uv, 0, SdfA, Dim);
impl_passthrough_op_2!(ProxyColorOp, Color, 1, SdfB, Dim);

pub type ProxyColor<SdfA, SdfB> = Operator<ProxyColorOp, (SdfA, SdfB)>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{prelude::{Point, ProxyColor}, test_op_attrs};

    #[test]
    fn test_sdf_color() {
        ProxyColor::<Point, Point>::default();
    }

    test_op_attrs!(ProxyColor::<Point, Point>);
}
