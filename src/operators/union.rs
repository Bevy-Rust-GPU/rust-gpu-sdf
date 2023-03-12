//! Compute the boolean union of two distance fields.

use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

/// Compute the boolean union of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct UnionOp<Sdf> {
    pub sdf: Sdf,
}

impl<SdfB, Dim> SignedDistanceOperator<Dim, Distance> for UnionOp<SdfB>
where
    SdfB: SignedDistanceField<Dim, Distance>,
    Dim: Clone,
{
    fn operator<SdfA>(&self, sdf: &SdfA, p: Dim) -> Distance
    where
        SdfA: SignedDistanceField<Dim, Distance>,
    {
        sdf.evaluate(p.clone())
            .min(*self.sdf.evaluate(p.clone()))
            .into()
    }
}

/// Compute the boolean union of two distance fields.
pub type Union<SdfA, SdfB> = Operator<SdfA, UnionOp<SdfB>>;

#[allow(non_camel_case_types)]
pub type Union_Sdf = (crate::operators::Operator_Op, UnionOp_Sdf);

impl<SdfA, SdfB> Union<SdfA, SdfB> {
    pub const SDF: Union_Sdf = (Operator::<(), ()>::OP, UnionOp::<()>::SDF);
}

#[cfg(test)]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::{Cube, Sphere};

    use super::Union;

    #[test]
    fn test_union() {
        Union::<Sphere, Cube>::default().with(Union::<(), ()>::SDF, Cube::default());
    }
}
