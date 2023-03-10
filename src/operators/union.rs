//! Compute the boolean union of two distance fields.

use rust_gpu_bridge::prelude::Vec3;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Compute the boolean union of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnionOp<Sdf>
where
    Sdf: SignedDistanceField<Vec3>,
{
    pub sdf: Sdf,
}

impl<SdfB> SignedDistanceOperator<Vec3> for UnionOp<SdfB>
where
    SdfB: SignedDistanceField<Vec3>,
{
    fn operator<SdfA>(&self, sdf: SdfA, p: Vec3) -> f32
    where
        SdfA: SignedDistanceField<Vec3>,
    {
        sdf.distance(p).min(self.sdf.distance(p))
    }
}

/// Compute the boolean union of two distance fields.
pub type Union<SdfA, SdfB> = Operator<SdfA, UnionOp<SdfB>, Vec3>;

