//! Displace the output of a distance field using the output of another distance field.

use type_fields::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{
        items::position::Position, AttrColor, AttrDistance, AttrNormal, AttrTangent, AttrUv,
        Distance, Field, FieldOperator, Operator,
    },
};

/// Displace the output of a distance field using the output of another distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct DisplaceOp {
    pub delta: f32,
}

impl<SdfA, Input> FieldOperator<SdfA, AttrDistance<Input>> for DisplaceOp
where
    SdfA: Field<AttrDistance<Input>>,
    Input: Clone,
{
    fn operator(&self, sdf_a: &SdfA, input: &Position<Input>) -> Distance {
        sdf_a.field(input) + self.delta
    }
}

impl_passthrough_op_2!(DisplaceOp, AttrNormal<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(DisplaceOp, AttrTangent<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(DisplaceOp, AttrUv<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(DisplaceOp, AttrColor<Dim>, 0, SdfA, Dim);

/// Displace the output of a distance field using the output of another distance field.
pub type Displace<SdfA, SdfB> = Operator<DisplaceOp, (SdfA, SdfB)>;

impl<SdfA, SdfB> Displace<SdfA, SdfB> {
    pub fn delta(&mut self) -> &mut f32 {
        self.op().delta()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use crate::{
        prelude::{Cube, DisplaceProxy, Point, Sphere},
        test_op_attrs,
    };
    use type_fields::field::Field;

    #[test]
    fn test_displace() {
        DisplaceProxy::<Cube, Sphere>::default().with(DisplaceProxy::displace, Sphere::default());
    }

    test_op_attrs!(DisplaceProxy::<Point, Point>);
}
