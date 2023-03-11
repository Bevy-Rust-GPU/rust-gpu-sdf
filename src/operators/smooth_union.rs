//! Compute the blended boolean union of two distance fields.

use rust_gpu_bridge::mix::Mix;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Compute the blended boolean union of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct SmoothUnionOp<Sdf> {
    pub sdf: Sdf,
    pub k: f32,
}

impl<SdfB, Dim> SignedDistanceOperator<Dim> for SmoothUnionOp<SdfB>
where
    SdfB: SignedDistanceField<Dim>,
    Dim: Clone,
{
    fn operator<SdfA>(&self, sdf: &SdfA, p: Dim) -> f32
    where
        SdfA: SignedDistanceField<Dim>,
    {
        let d1 = sdf.distance(p.clone());
        let d2 = self.sdf.distance(p.clone());
        let h = (0.5 + 0.5 * (d2 - d1) / self.k).clamp(0.0, 1.0);
        d2.mix(d1, h) - self.k * h * (1.0 - h)
    }
}

/// Compute the blended boolean union of two distance fields.
pub type SmoothUnion<SdfA, SdfB, Dim> = Operator<SdfA, SmoothUnionOp<SdfB>, Dim>;
