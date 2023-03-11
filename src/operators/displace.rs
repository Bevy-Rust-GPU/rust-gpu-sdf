//! Displace the output of a distance field using the output of another distance field.

use crate::operators::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Displace the output of a distance field using the output of another distance field.
pub struct DisplaceOp<Sdf>
{
    pub displace: Sdf,
}

impl<SdfB, Dim> SignedDistanceOperator<Dim> for DisplaceOp<SdfB>
where
    SdfB: SignedDistanceField<Dim>,
{
    fn operator<SdfA>(&self, sdf: &SdfA, p: Dim) -> f32
    where
        SdfA: SignedDistanceField<Dim>,
        Dim: Clone,
    {
        sdf.distance(p.clone()) + self.displace.distance(p)
    }
}

/// Displace the output of a distance field using the output of another distance field.
pub type Displace<SdfA, SdfB, Dim> = Operator<SdfA, DisplaceOp<SdfB>, Dim>;
