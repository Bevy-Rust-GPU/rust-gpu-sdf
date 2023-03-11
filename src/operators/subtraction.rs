//! Compute the boolean subtraction of two distance fields.

use core::ops::Neg;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Compute the boolean subtraction of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
pub type Subtraction<SdfA, SdfB, Dim> = Operator<SdfA, SubtractionOp<SdfB>, Dim>;
