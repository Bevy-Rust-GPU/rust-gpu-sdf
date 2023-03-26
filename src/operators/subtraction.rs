//! Compute the boolean subtraction of two distance fields.

use core::ops::Neg;

use rust_gpu_bridge::glam::Vec2;
use type_fields::Field;

use crate::{
    prelude::{Distance, FieldFunction, Operator, SignedDistanceOperator},
    signed_distance_field::attributes::{normal::Normal, uv::Uv},
};

/// Compute the boolean subtraction of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct SubtractionOp<Sdf> {
    pub sdf: Sdf,
}

impl<SdfA, SdfB, Dim> SignedDistanceOperator<SdfA, Dim, Distance> for SubtractionOp<SdfB>
where
    SdfA: FieldFunction<Dim, Distance>,
    SdfB: FieldFunction<Dim, Distance>,
    Dim: Clone,
{
    fn operator(&self, attr: Distance, sdf: &SdfA, p: Dim) -> f32 {
        sdf.evaluate(attr, p.clone())
            .neg()
            .max(self.sdf.evaluate(attr, p))
    }
}

impl<SdfA, SdfB, Dim> SignedDistanceOperator<SdfA, Dim, Normal<Dim>> for SubtractionOp<SdfB>
where
    SdfA: FieldFunction<Dim, Distance>,
    SdfA: FieldFunction<Dim, Normal<Dim>>,
    SdfB: FieldFunction<Dim, Distance>,
    SdfB: FieldFunction<Dim, Normal<Dim>>,
    Dim: Clone,
{
    fn operator(&self, attr: Normal<Dim>, sdf: &SdfA, p: Dim) -> Dim {
        let dist_a = sdf.evaluate(Distance, p.clone());
        let dist_b = self.sdf.evaluate(Distance, p.clone());

        if -dist_a > dist_b {
            sdf.evaluate(attr, p)
        } else {
            self.sdf.evaluate(attr, p)
        }
    }
}

impl<SdfA, SdfB, Dim> SignedDistanceOperator<SdfA, Dim, Uv> for SubtractionOp<SdfB>
where
    SdfA: FieldFunction<Dim, Distance>,
    SdfA: FieldFunction<Dim, Uv>,
    SdfB: FieldFunction<Dim, Distance>,
    SdfB: FieldFunction<Dim, Uv>,
    Dim: Clone,
{
    fn operator(&self, attr: Uv, sdf: &SdfA, p: Dim) -> Vec2 {
        let dist_a = sdf.evaluate(Distance, p.clone());
        let dist_b = self.sdf.evaluate(Distance, p.clone());

        if dist_a < dist_b {
            sdf.evaluate(attr, p)
        } else {
            self.sdf.evaluate(attr, p)
        }
    }
}

/// Compute the boolean subtraction of two distance fields.
pub type Subtraction<SdfA, SdfB> = Operator<SubtractionOp<SdfB>, SdfA>;

impl<SdfA, SdfB> Subtraction<SdfA, SdfB> {
    pub fn sdf(&mut self) -> &mut SdfB {
        &mut self.op.sdf
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::{Cube, Sphere};

    use super::Subtraction;

    #[test]
    fn test_subtraction() {
        Subtraction::<Cube, Sphere>::default().with(Subtraction::sdf, Sphere::default());
    }
}
