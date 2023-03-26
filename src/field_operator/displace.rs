//! Displace the output of a distance field using the output of another distance field.

use core::ops::Add;

use type_fields::Field;

use crate::prelude::{Distance, FieldFunction, FieldOperator, Normal, Operator, Uv};

use rust_gpu_bridge::{glam::Vec2, Normalize};

/// Displace the output of a distance field using the output of another distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct DisplaceOp<Sdf> {
    pub displace: Sdf,
}

impl<SdfA, SdfB, Dim> FieldOperator<SdfA, Dim, Distance> for DisplaceOp<SdfB>
where
    SdfA: FieldFunction<Dim, Distance>,
    SdfB: FieldFunction<Dim, Distance>,
    Dim: Clone,
{
    fn operator(&self, attr: Distance, sdf: &SdfA, p: Dim) -> f32 {
        sdf.evaluate(attr, p.clone())
            .add(self.displace.evaluate(attr, p))
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<SdfA, Dim, Normal<Dim>> for DisplaceOp<SdfB>
where
    SdfA: FieldFunction<Dim, Normal<Dim>>,
    SdfB: FieldFunction<Dim, Normal<Dim>>,
    Dim: Clone + Add<Dim, Output = Dim> + Normalize,
{
    fn operator(&self, attr: Normal<Dim>, sdf: &SdfA, p: Dim) -> Dim {
        sdf.evaluate(attr, p.clone())
            .clone()
            .add(self.displace.evaluate(attr, p).clone())
            .normalize()
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<SdfA, Dim, Uv> for DisplaceOp<SdfB>
where
    SdfA: FieldFunction<Dim, Uv>,
    SdfB: FieldFunction<Dim, Uv>,
    Dim: Clone + Add<Dim, Output = Dim> + Normalize,
{
    fn operator(&self, attr: Uv, sdf: &SdfA, p: Dim) -> Vec2 {
        sdf.evaluate(attr, p.clone())
            .clone()
            .add(self.displace.evaluate(attr, p).clone())
            .normalize()
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
