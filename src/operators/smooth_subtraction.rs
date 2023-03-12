//! Compute the blended boolean subtraction of two distance fields.
use rust_gpu_bridge::mix::Mix;
use type_fields::Field;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Compute the blended boolean subtraction of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct SmoothSubtractionOp<Sdf> {
    pub sdf: Sdf,
    pub k: f32,
}

impl<SdfB, Dim> SignedDistanceOperator<Dim> for SmoothSubtractionOp<SdfB>
where
    SdfB: SignedDistanceField<Dim, f32>,
    Dim: Clone,
{
    fn operator<SdfA>(&self, sdf: &SdfA, p: Dim) -> f32
    where
        SdfA: SignedDistanceField<Dim, f32>,
    {
        let d1 = sdf.evaluate(p.clone());
        let d2 = self.sdf.evaluate(p.clone());
        let h = (0.5 - 0.5 * (d2 + d1) / self.k).clamp(0.0, 1.0);
        d2.mix(-d1, h) + self.k * h * (1.0 - h)
    }
}

/// Compute the blended boolean subtraction of two distance fields.
pub type SmoothSubtraction<SdfA, SdfB> = Operator<SdfA, SmoothSubtractionOp<SdfB>>;

#[allow(non_camel_case_types)]
pub type SmoothSubtraction_Sdf = (crate::operators::Operator_Op, SmoothSubtractionOp_Sdf);

#[allow(non_camel_case_types)]
pub type SmoothSubtraction_K = (crate::operators::Operator_Op, SmoothSubtractionOp_K);

impl<SdfA, SdfB> SmoothSubtraction<SdfA, SdfB> {
    pub const SDF: SmoothSubtraction_Sdf = (Operator::<(), ()>::OP, SmoothSubtractionOp::<()>::SDF);

    pub const K: SmoothSubtraction_K = (Operator::<(), ()>::OP, SmoothSubtractionOp::<()>::K);
}

#[cfg(test)]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::{Cube, Sphere};

    use super::SmoothSubtraction;

    #[test]
    fn test_smooth_subtraction() {
        SmoothSubtraction::<Cube, Sphere>::default()
            .with(SmoothSubtraction::<(), ()>::SDF, Sphere::default())
            .with(SmoothSubtraction::<(), ()>::K, f32::default());
    }
}
