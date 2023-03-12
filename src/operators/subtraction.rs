//! Compute the boolean subtraction of two distance fields.

use core::ops::Neg;

use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

/// Compute the boolean subtraction of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct SubtractionOp<Sdf> {
    pub sdf: Sdf,
}

impl<SdfB, Dim> SignedDistanceOperator<Dim, Distance> for SubtractionOp<SdfB>
where
    SdfB: SignedDistanceField<Dim, Distance>,
    Dim: Clone,
{
    fn operator<SdfA>(&self, sdf: &SdfA, p: Dim) -> Distance
    where
        SdfA: SignedDistanceField<Dim, Distance>,
    {
        sdf.evaluate(p.clone())
            .neg()
            .max(*self.sdf.evaluate(p.clone()))
            .into()
    }
}

/// Compute the boolean subtraction of two distance fields.
pub type Subtraction<SdfA, SdfB> = Operator<SdfA, SubtractionOp<SdfB>>;

#[allow(non_camel_case_types)]
pub type Subtraction_Sdf = (crate::operators::Operator_Op, SubtractionOp_Sdf);

impl<SdfA, SdfB> Subtraction<SdfA, SdfB> {
    pub const SDF: Subtraction_Sdf = (Operator::<(), ()>::OP, SubtractionOp::<()>::SDF);
}

#[cfg(test)]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::{Cube, Sphere};

    use super::Subtraction;

    #[test]
    fn test_subtraction() {
        Subtraction::<Cube, Sphere>::default().with(Subtraction::<(), ()>::SDF, Sphere::default());
    }
}
