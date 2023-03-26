//! Compute the boolean union of two distance fields.

use rust_gpu_bridge::glam::Vec2;
use type_fields::Field;

use crate::prelude::{Distance, FieldFunction, FieldOperator, Normal, Operator, Uv};

/// Compute the boolean union of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct UnionOp<Sdf> {
    pub sdf: Sdf,
}

impl<SdfA, SdfB, Dim> FieldOperator<SdfA, Dim, Distance> for UnionOp<SdfB>
where
    SdfA: FieldFunction<Dim, Distance>,
    SdfB: FieldFunction<Dim, Distance>,
    Dim: Clone,
{
    fn operator(&self, attr: Distance, sdf: &SdfA, p: Dim) -> f32 {
        sdf.evaluate(attr, p.clone())
            .min(self.sdf.evaluate(attr, p))
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<SdfA, Dim, Normal<Dim>> for UnionOp<SdfB>
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

        if dist_a < dist_b {
            sdf.evaluate(attr, p)
        } else {
            self.sdf.evaluate(attr, p)
        }
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<SdfA, Dim, Uv> for UnionOp<SdfB>
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

/// Compute the boolean union of two distance fields.
pub type Union<SdfA, SdfB> = Operator<UnionOp<SdfB>, SdfA>;

impl<SdfA, SdfB> Union<SdfA, SdfB> {
    pub fn sdf(&mut self) -> &mut SdfB {
        &mut self.op.sdf
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::{Cube, Sphere};

    use super::Union;

    #[test]
    fn test_union() {
        Union::<Sphere, Cube>::default().with(Union::sdf, Cube::default());
    }
}
