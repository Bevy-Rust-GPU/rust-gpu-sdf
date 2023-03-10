//! Compute the boolean subtraction of two distance fields.

use core::ops::Neg;

use rust_gpu_bridge::prelude::Vec3;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Compute the boolean subtraction of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubtractionOp<Sdf>
where
    Sdf: SignedDistanceField<Vec3>,
{
    pub sdf: Sdf,
}

impl<SdfB> SignedDistanceOperator<Vec3> for SubtractionOp<SdfB>
where
    SdfB: SignedDistanceField<Vec3>,
{
    fn operator<SdfA>(&self, sdf: SdfA, p: Vec3) -> f32
    where
        SdfA: SignedDistanceField<Vec3>,
    {
        sdf.distance(p).neg().max(self.sdf.distance(p))
    }
}

/// Compute the boolean subtraction of two distance fields.
pub type Subtraction<SdfA, SdfB> = Operator<SdfA, SubtractionOp<SdfB>, Vec3>;
