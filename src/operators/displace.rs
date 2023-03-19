//! Displace the output of a distance field using the output of another distance field.

use core::ops::Add;

use type_fields::Field;

use crate::{
    prelude::{Distance, Operator, DistanceFunction, SignedDistanceOperator},
    signed_distance_field::attributes::{normal::Normal, uv::Uv},
};

use rust_gpu_bridge::prelude::Normalize;

/// Displace the output of a distance field using the output of another distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct DisplaceOp<Sdf> {
    pub displace: Sdf,
}

impl<SdfA, SdfB, Dim> SignedDistanceOperator<SdfA, Dim, Distance> for DisplaceOp<SdfB>
where
    SdfA: DistanceFunction<Dim, Distance>,
    SdfB: DistanceFunction<Dim, Distance>,
    Dim: Clone,
{
    fn operator(&self, sdf: &SdfA, p: Dim) -> Distance {
        (*sdf.evaluate(p.clone()))
            .add(*self.displace.evaluate(p))
            .into()
    }
}

impl<SdfA, SdfB, Dim> SignedDistanceOperator<SdfA, Dim, Normal<Dim>> for DisplaceOp<SdfB>
where
    SdfA: DistanceFunction<Dim, Normal<Dim>>,
    SdfB: DistanceFunction<Dim, Normal<Dim>>,
    Dim: Clone + Add<Dim, Output = Dim> + Normalize,
{
    fn operator(&self, sdf: &SdfA, p: Dim) -> Normal<Dim> {
        (*sdf.evaluate(p.clone()))
            .clone()
            .add((*self.displace.evaluate(p)).clone())
            .normalize()
            .into()
    }
}

impl<SdfA, SdfB, Dim> SignedDistanceOperator<SdfA, Dim, Uv> for DisplaceOp<SdfB>
where
    SdfA: DistanceFunction<Dim, Uv>,
    SdfB: DistanceFunction<Dim, Uv>,
    Dim: Clone + Add<Dim, Output = Dim> + Normalize,
{
    fn operator(&self, sdf: &SdfA, p: Dim) -> Uv {
        (*sdf.evaluate(p.clone()))
            .clone()
            .add((*self.displace.evaluate(p)).clone())
            .normalize()
            .into()
    }
}

/// Displace the output of a distance field using the output of another distance field.
pub type Displace<SdfA, SdfB> = Operator<DisplaceOp<SdfB>, SdfA>;
impl<SdfA, SdfB> Displace<SdfA, SdfB> {
    pub fn displace(&mut self) -> &mut SdfB {
        &mut self.op.displace
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::{Cube, Sphere};

    use super::Displace;

    #[test]
    fn test_displace() {
        Displace::<Cube, Sphere>::default().with(Displace::displace, Sphere::default());
    }
}
