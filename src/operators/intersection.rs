//! Compute the boolean intersection of two distance fields.

use rust_gpu_bridge::prelude::Vec3;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Compute the boolean intersection of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntersectionOp<Sdf>
where
    Sdf: SignedDistanceField<Vec3>,
{
    pub sdf: Sdf,
}

impl<SdfB> SignedDistanceOperator<Vec3> for IntersectionOp<SdfB>
where
    SdfB: SignedDistanceField<Vec3>,
{
    fn operator<SdfA>(&self, sdf: SdfA, p: Vec3) -> f32
    where
        SdfA: SignedDistanceField<Vec3>,
    {
        sdf.distance(p).max(self.sdf.distance(p))
    }
}

/// Compute the boolean intersection of two distance fields.
pub type Intersection<SdfA, SdfB> = Operator<SdfA, IntersectionOp<SdfB>, Vec3>;

