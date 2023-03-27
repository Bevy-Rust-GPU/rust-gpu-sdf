//! Override the colors of an SDF with the colors of another SDF

use type_fields::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{Color, Distance, Normal, Tangent, Uv},
};

use super::{FieldOperator, Operator};

/// Override the colors of an SDF with the colors of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct SdfColorOp;

impl_passthrough_op_2!(SdfColorOp, <Dim>, Distance, 0);
impl_passthrough_op_2!(SdfColorOp, <Dim>, Normal<Dim>, 0);
impl_passthrough_op_2!(SdfColorOp, <Dim>, Tangent<Dim>, 0);
impl_passthrough_op_2!(SdfColorOp, <Dim>, Uv, 0);
impl_passthrough_op_2!(SdfColorOp, <Dim>, Color, 1);

pub type SdfColor<SdfA, SdfB> = Operator<SdfColorOp, (SdfA, SdfB)>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{prelude::Point, test_op_attrs};

    use super::SdfColor;

    #[test]
    fn test_sdf_color() {
        SdfColor::<Point, Point>::default();
    }

    test_op_attrs!(SdfColor::<Point, Point>);
}
