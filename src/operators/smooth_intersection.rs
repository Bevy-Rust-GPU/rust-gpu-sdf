//! Compute the blended boolean intersection of two distance fields.

use rust_gpu_bridge::{mix::Mix, prelude::Vec3};

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Compute the blended boolean intersection of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct SmoothIntersectionOp<Sdf>
where
    Sdf: SignedDistanceField<Vec3>,
{
    pub sdf: Sdf,
    pub k: f32,
}

impl<SdfB> SignedDistanceOperator<Vec3> for SmoothIntersectionOp<SdfB>
where
    SdfB: SignedDistanceField<Vec3>,
{
    fn operator<SdfA>(&self, sdf: SdfA, p: Vec3) -> f32
    where
        SdfA: SignedDistanceField<Vec3>,
    {
        let d1 = sdf.distance(p);
        let d2 = self.sdf.distance(p);
        let h = (0.5 - 0.5 * (d2 - d1) / self.k).clamp(0.0, 1.0);
        d2.mix(d1, h) + self.k * h * (1.0 - h)
    }
}

/// Compute the blended boolean intersection of two distance fields.
pub type SmoothIntersection<SdfA, SdfB> = Operator<SdfA, SmoothIntersectionOp<SdfB>, Vec3>;
