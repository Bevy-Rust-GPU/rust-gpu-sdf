//! Displace the output of a distance field using the output of another distance field.

use type_fields::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{Color, Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv},
};

/// Displace the output of a distance field using the output of another distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct DisplaceOp {
    pub delta: f32,
}

impl<SdfA, Dim> FieldOperator<SdfA, Dim, Distance> for DisplaceOp
where
    SdfA: Field<Dim, Distance>,
    Dim: Clone,
{
    fn operator(&self, attr: Distance, sdf_a: &SdfA, p: Dim) -> f32 {
        sdf_a.field(attr, p.clone()) + self.delta
    }
}

impl_passthrough_op_2!(DisplaceOp, Normal<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(DisplaceOp, Tangent<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(DisplaceOp, Uv, 0, SdfA, Dim);
impl_passthrough_op_2!(DisplaceOp, Color, 0, SdfA, Dim);

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
