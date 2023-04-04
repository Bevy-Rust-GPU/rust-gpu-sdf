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
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct DisplaceProxyOp;

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), AttrDistance<Input>> for DisplaceProxyOp
where
    SdfA: Field<AttrDistance<Input>>,
    SdfB: Field<AttrDistance<Input>>,
    Input: Clone,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), input: &Position<Input>) -> Distance {
        sdf_a.field(input) + *sdf_b.field(input)
    }
}

impl_passthrough_op_2!(DisplaceProxyOp, AttrNormal<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(DisplaceProxyOp, AttrTangent<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(DisplaceProxyOp, AttrUv<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(DisplaceProxyOp, AttrColor<Dim>, 0, SdfA, Dim);

/// Displace the output of a distance field using the output of another distance field.
pub type DisplaceProxy<SdfA, SdfB> = Operator<DisplaceProxyOp, (SdfA, SdfB)>;

impl<SdfA, SdfB> DisplaceProxy<SdfA, SdfB> {
    pub fn sdf(&mut self) -> &mut SdfA {
        &mut self.target.0
    }

    pub fn displace(&mut self) -> &mut SdfB {
        &mut self.target.1
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
