//! Compute the boolean union of two distance fields.

use type_fields::Field;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Compute the boolean union of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct UnionOp<Sdf> {
    pub sdf: Sdf,
}

impl<SdfB, Dim> SignedDistanceOperator<Dim> for UnionOp<SdfB>
where
    SdfB: SignedDistanceField<Dim>,
    Dim: Clone,
{
    fn operator<SdfA>(&self, sdf: &SdfA, p: Dim) -> f32
    where
        SdfA: SignedDistanceField<Dim>,
    {
        sdf.distance(p.clone()).min(self.sdf.distance(p.clone()))
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
