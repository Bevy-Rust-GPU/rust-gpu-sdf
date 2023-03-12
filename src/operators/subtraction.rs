//! Compute the boolean subtraction of two distance fields.

use core::ops::Neg;

use type_fields::Field;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Compute the boolean subtraction of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct SubtractionOp<Sdf> {
    pub sdf: Sdf,
}

impl<SdfB, Dim> SignedDistanceOperator<Dim> for SubtractionOp<SdfB>
where
    SdfB: SignedDistanceField<Dim>,
    Dim: Clone,
{
    fn operator<SdfA>(&self, sdf: &SdfA, p: Dim) -> f32
    where
        SdfA: SignedDistanceField<Dim>,
    {
        sdf.distance(p.clone())
            .neg()
            .max(self.sdf.distance(p.clone()))
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
