//! Compute the boolean union of two distance fields.

use rust_gpu_bridge::glam::Vec2;
use type_fields::Field;

use crate::prelude::{Distance, FieldFunction, FieldOperator, Normal, Operator, Uv};

/// Compute the boolean union of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct UnionOp;

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Distance> for UnionOp
where
    SdfA: FieldFunction<Dim, Distance>,
    SdfB: FieldFunction<Dim, Distance>,
    Dim: Clone,
{
    fn operator(&self, attr: Distance, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> f32 {
        sdf_a.field(attr, p.clone()).min(sdf_b.field(attr, p))
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Normal<Dim>> for UnionOp
where
    SdfA: FieldFunction<Dim, Distance>,
    SdfA: FieldFunction<Dim, Normal<Dim>>,
    SdfB: FieldFunction<Dim, Distance>,
    SdfB: FieldFunction<Dim, Normal<Dim>>,
    Dim: Clone,
{
    fn operator(&self, attr: Normal<Dim>, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> Dim {
        let dist_a = sdf_a.field(Distance, p.clone());
        let dist_b = sdf_b.field(Distance, p.clone());

        if dist_a < dist_b {
            sdf_a.field(attr, p)
        } else {
            sdf_b.field(attr, p)
        }
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Uv> for UnionOp
where
    SdfA: FieldFunction<Dim, Distance>,
    SdfA: FieldFunction<Dim, Uv>,
    SdfB: FieldFunction<Dim, Distance>,
    SdfB: FieldFunction<Dim, Uv>,
    Dim: Clone,
{
    fn operator(&self, attr: Uv, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> Vec2 {
        let dist_a = sdf_a.field(Distance, p.clone());
        let dist_b = sdf_b.field(Distance, p.clone());

        if dist_a < dist_b {
            sdf_a.field(attr, p)
        } else {
            sdf_b.field(attr, p)
        }
    }
}

/// Compute the boolean union of two distance fields.
pub type Union<SdfA, SdfB> = Operator<UnionOp, (SdfA, SdfB)>;

impl<SdfA, SdfB> Union<SdfA, SdfB> {
    pub fn sdf_a(&mut self) -> &mut SdfA {
        &mut self.target().0
    }

    pub fn sdf_b(&mut self) -> &mut SdfB {
        &mut self.target().1
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{
        prelude::{Cube, Point, Sphere, Union},
        test_op_attrs,
    };

    #[test]
    fn test_union() {
        Union::<Sphere, Cube>::default();
    }

    test_op_attrs!(Union::<Point, Point>);
}
