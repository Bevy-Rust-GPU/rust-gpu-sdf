//! Compute the blended boolean union of two distance fields.

use core::ops::Sub;

use rust_gpu_bridge::mix::Mix;
use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

/// Compute the blended boolean union of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct SmoothUnionOp<Sdf> {
    pub sdf: Sdf,
    pub k: f32,
}

impl<SdfB, Dim> SignedDistanceOperator<Dim, Distance> for SmoothUnionOp<SdfB>
where
    SdfB: SignedDistanceField<Dim, Distance>,
    Dim: Clone,
{
    fn operator<SdfA>(&self, sdf: &SdfA, p: Dim) -> Distance
    where
        SdfA: SignedDistanceField<Dim, Distance>,
    {
        let d1 = *sdf.evaluate(p.clone());
        let d2 = *self.sdf.evaluate(p);
        let h = (0.5 + 0.5 * (d2 - d1) / self.k).clamp(0.0, 1.0);
        d2.mix(d1, h).sub(self.k * h * (1.0 - h)).into()
    }
}

/// Compute the blended boolean union of two distance fields.
pub type SmoothUnion<SdfA, SdfB> = Operator<SmoothUnionOp<SdfB>, SdfA>;

impl<SdfA, SdfB> SmoothUnion<SdfA, SdfB> {
    pub fn sdf(&mut self) -> &mut SdfB {
        &mut self.op.sdf
    }

    pub fn k(&mut self) -> &mut f32 {
        &mut self.op.k
    }
}

#[cfg(test)]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::{Cube, Sphere};

    use super::SmoothUnion;

    #[test]
    fn test_smooth_union() {
        SmoothUnion::<Cube, Sphere>::default()
            .with(SmoothUnion::sdf, Sphere::default())
            .with(SmoothUnion::k, f32::default());
    }
}
