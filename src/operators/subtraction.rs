//! Compute the boolean subtraction of two distance fields.

use core::ops::Neg;

use type_fields::Field;

use crate::{
    prelude::{Distance, Operator, DistanceFunction, SignedDistanceOperator},
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
    SdfA: DistanceFunction<Dim, Distance>,
    SdfB: DistanceFunction<Dim, Distance>,
    Dim: Clone,
{
    fn operator(&self, sdf: &SdfA, p: Dim) -> Distance {
        sdf.evaluate(p.clone())
            .neg()
            .max(*self.sdf.evaluate(p))
            .into()
    }
}

impl<SdfA, SdfB, Dim> SignedDistanceOperator<SdfA, Dim, Normal<Dim>> for SubtractionOp<SdfB>
where
    SdfA: DistanceFunction<Dim, Distance>,
    SdfA: DistanceFunction<Dim, Normal<Dim>>,
    SdfB: DistanceFunction<Dim, Distance>,
    SdfB: DistanceFunction<Dim, Normal<Dim>>,
    Dim: Clone,
{
    fn operator(&self, sdf: &SdfA, p: Dim) -> Normal<Dim> {
        let dist_a: Distance = sdf.evaluate(p.clone());
        let dist_b: Distance = self.sdf.evaluate(p.clone());

        let n: Normal<Dim> = if -*dist_a > *dist_b {
            sdf.evaluate(p)
        } else {
            self.sdf.evaluate(p)
        };

        n.into()
    }
}

impl<SdfA, SdfB, Dim> SignedDistanceOperator<SdfA, Dim, Uv> for SubtractionOp<SdfB>
where
    SdfA: DistanceFunction<Dim, Distance>,
    SdfA: DistanceFunction<Dim, Uv>,
    SdfB: DistanceFunction<Dim, Distance>,
    SdfB: DistanceFunction<Dim, Uv>,
    Dim: Clone,
{
    fn operator(&self, sdf: &SdfA, p: Dim) -> Uv {
        let dist_a: Distance = sdf.evaluate(p.clone());
        let dist_b: Distance = self.sdf.evaluate(p.clone());

        let uv: Uv = if *dist_a < *dist_b {
            sdf.evaluate(p)
        } else {
            self.sdf.evaluate(p)
        };

        uv.into()
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
