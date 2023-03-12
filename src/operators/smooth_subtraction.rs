//! Compute the blended boolean subtraction of two distance fields.
use core::ops::{Add, Mul};

use rust_gpu_bridge::mix::Mix;
use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

/// Compute the blended boolean subtraction of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct SmoothSubtractionOp<Sdf> {
    pub sdf: Sdf,
    pub k: f32,
}

impl<SdfB, Dim> SignedDistanceOperator<Dim, Distance> for SmoothSubtractionOp<SdfB>
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
        let h = (0.5 - 0.5 * (d2 + d1) / self.k).clamp(0.0, 1.0);
        d2.mix(-d1, h).add(self.k.mul(h).mul(1.0 - h)).into()
    }
}

/// Compute the blended boolean subtraction of two distance fields.
pub type SmoothSubtraction<SdfA, SdfB> = Operator<SdfA, SmoothSubtractionOp<SdfB>>;

impl<SdfA, SdfB> SmoothSubtraction<SdfA, SdfB> {
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

    use super::SmoothSubtraction;

    #[test]
    fn test_smooth_subtraction() {
        SmoothSubtraction::<Cube, Sphere>::default()
            .with(SmoothSubtraction::sdf, Sphere::default())
            .with(SmoothSubtraction::k, f32::default());
    }
}
