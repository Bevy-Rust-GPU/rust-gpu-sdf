//! Compute the blended boolean subtraction of two distance fields.
use core::ops::{Add, Mul};

use rust_gpu_bridge::Mix;
use type_fields::Field;

use crate::prelude::{Distance, FieldFunction, Operator, SignedDistanceOperator};

/// Compute the blended boolean subtraction of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct SmoothSubtractionOp<Sdf> {
    pub sdf: Sdf,
    pub k: f32,
}

impl<SdfA, SdfB, Dim> SignedDistanceOperator<SdfA, Dim, Distance> for SmoothSubtractionOp<SdfB>
where
    SdfA: FieldFunction<Dim, Distance>,
    SdfB: FieldFunction<Dim, Distance>,
    Dim: Clone,
{
    fn operator(&self, attr: Distance, sdf: &SdfA, p: Dim) -> f32 {
        let d1 = sdf.evaluate(attr, p.clone());
        let d2 = self.sdf.evaluate(attr, p);
        let h = (0.5 - 0.5 * (d2 + d1) / self.k).clamp(0.0, 1.0);
        d2.mix(-d1, h).add(self.k.mul(h).mul(1.0 - h)).into()
    }
}

/// Compute the blended boolean subtraction of two distance fields.
pub type SmoothSubtraction<SdfA, SdfB> = Operator<SmoothSubtractionOp<SdfB>, SdfA>;

impl<SdfA, SdfB> SmoothSubtraction<SdfA, SdfB> {
    pub fn sdf(&mut self) -> &mut SdfB {
        &mut self.op.sdf
    }

    pub fn k(&mut self) -> &mut f32 {
        &mut self.op.k
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
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
