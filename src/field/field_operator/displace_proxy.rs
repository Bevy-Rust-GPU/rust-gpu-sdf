//! Displace the output of a distance field using the output of another distance field.

use type_fields::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{Color, Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv},
};

/// Displace the output of a distance field using the output of another distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct DisplaceProxyOp;

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Distance> for DisplaceProxyOp
where
    SdfA: Field<Dim, Distance>,
    SdfB: Field<Dim, Distance>,
    Dim: Clone,
{
    fn operator(&self, attr: Distance, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> f32 {
        sdf_a.field(attr, p.clone()) + sdf_b.field(attr, p)
    }
}

impl_passthrough_op_2!(DisplaceProxyOp, Normal<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(DisplaceProxyOp, Tangent<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(DisplaceProxyOp, Uv, 0, SdfA, Dim);
impl_passthrough_op_2!(DisplaceProxyOp, Color, 0, SdfA, Dim);

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
