//! Compute the blended boolean intersection of two distance fields.

use core::ops::{Add, Mul};

use rust_gpu_bridge::mix::Mix;
use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

/// Compute the blended boolean intersection of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct SmoothIntersectionOp<Sdf> {
    pub sdf: Sdf,
    pub k: f32,
}

impl<SdfB, Dim> SignedDistanceOperator<Dim, Distance> for SmoothIntersectionOp<SdfB>
where
    SdfB: SignedDistanceField<Dim, Distance>,
    Dim: Clone,
{
    fn operator<SdfA>(&self, sdf: &SdfA, p: Dim) -> Distance
    where
        SdfA: SignedDistanceField<Dim, Distance>,
    {
        let d1 = *sdf.evaluate(p.clone());
        let d2 = *self.sdf.evaluate(p.clone());
        let h = (0.5 - 0.5 * (d2 - d1) / self.k).clamp(0.0, 1.0);
        d2.mix(d1, h).add(self.k.mul(h).mul(1.0 - h)).into()
    }
}

/// Compute the blended boolean intersection of two distance fields.
pub type SmoothIntersection<SdfA, SdfB> = Operator<SdfA, SmoothIntersectionOp<SdfB>>;

#[allow(non_camel_case_types)]
pub type SmoothIntersection_Sdf = (crate::operators::Operator_Op, SmoothIntersectionOp_Sdf);

#[allow(non_camel_case_types)]
pub type SmoothIntersection_K = (crate::operators::Operator_Op, SmoothIntersectionOp_K);

impl<SdfA, SdfB> SmoothIntersection<SdfA, SdfB> {
    pub const SDF: SmoothIntersection_Sdf =
        (Operator::<(), ()>::OP, SmoothIntersectionOp::<()>::SDF);

    pub const K: SmoothIntersection_K = (Operator::<(), ()>::OP, SmoothIntersectionOp::<()>::K);
}

#[cfg(test)]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::{Cube, Sphere};

    use super::SmoothIntersection;

    #[test]
    fn test_smooth_intersection() {
        SmoothIntersection::<Cube, Sphere>::default()
            .with(SmoothIntersection::<(), ()>::SDF, Sphere::default())
            .with(SmoothIntersection::<(), ()>::K, f32::default());
    }
}
