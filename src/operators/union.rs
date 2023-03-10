//! Compute the boolean union of two distance fields.

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Compute the boolean union of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnionOp<Sdf> {
    pub sdf: Sdf,
}

impl<SdfB, Dim> SignedDistanceOperator<Dim> for UnionOp<SdfB>
where
    SdfB: SignedDistanceField<Dim>,
    Dim: Clone,
{
    fn operator<SdfA>(&self, sdf: SdfA, p: Dim) -> f32
    where
        SdfA: SignedDistanceField<Dim>,
    {
        sdf.distance(p.clone()).min(self.sdf.distance(p.clone()))
    }
}

/// Compute the boolean union of two distance fields.
pub type Union<SdfA, SdfB, Dim> = Operator<SdfA, UnionOp<SdfB>, Dim>;
