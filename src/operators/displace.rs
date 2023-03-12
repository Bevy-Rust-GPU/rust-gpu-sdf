//! Displace the output of a distance field using the output of another distance field.

use type_fields::Field;

use crate::operators::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Displace the output of a distance field using the output of another distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct DisplaceOp<Sdf> {
    pub displace: Sdf,
}

impl<SdfB, Dim> SignedDistanceOperator<Dim> for DisplaceOp<SdfB>
where
    SdfB: SignedDistanceField<Dim, f32>,
{
    fn operator<SdfA>(&self, sdf: &SdfA, p: Dim) -> f32
    where
        SdfA: SignedDistanceField<Dim, f32>,
        Dim: Clone,
    {
        sdf.evaluate(p.clone()) + self.displace.evaluate(p)
    }
}

/// Displace the output of a distance field using the output of another distance field.
pub type Displace<SdfA, SdfB> = Operator<SdfA, DisplaceOp<SdfB>>;

#[allow(non_camel_case_types)]
pub type Displace_Displace = (crate::operators::Operator_Op, DisplaceOp_Displace);

impl<SdfA, SdfB> Displace<SdfA, SdfB> {
    pub const DISPLACE: Displace_Displace =
        (Operator::<(), (),>::OP, DisplaceOp::<()>::DISPLACE);
}

#[cfg(test)]
pub mod tests {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::{Cube, Sphere};

    use super::Displace;

    #[test]
    fn test_displace() {
        Displace::<Cube, Sphere>::default()
            .with(Displace::<(), ()>::DISPLACE, Sphere::default());
    }
}
