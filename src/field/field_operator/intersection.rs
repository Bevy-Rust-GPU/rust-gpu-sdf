//! Compute the boolean intersection of two distance fields.

use rust_gpu_bridge::glam::Vec2;
use type_fields::Field;

use crate::prelude::{Distance, Field, FieldOperator, Normal, Operator, Uv};

/// Compute the boolean intersection of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct IntersectionOp;

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Distance> for IntersectionOp
where
    SdfA: Field<Dim, Distance>,
    SdfB: Field<Dim, Distance>,
    Dim: Clone,
{
    fn operator(&self, attr: Distance, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> f32 {
        sdf_a.field(attr, p.clone()).max(sdf_b.field(attr, p))
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Normal<Dim>> for IntersectionOp
where
    SdfA: Field<Dim, Distance>,
    SdfA: Field<Dim, Normal<Dim>>,
    SdfB: Field<Dim, Distance>,
    SdfB: Field<Dim, Normal<Dim>>,
    Dim: Clone,
{
    fn operator(&self, attr: Normal<Dim>, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> Dim {
        let dist_a = sdf_a.field(Distance, p.clone());
        let dist_b = sdf_b.field(Distance, p.clone());

        let n = if dist_a > dist_b {
            sdf_a.field(attr, p)
        } else {
            sdf_b.field(attr, p)
        };

        n.into()
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Uv> for IntersectionOp
where
    SdfA: Field<Dim, Distance>,
    SdfA: Field<Dim, Uv>,
    SdfB: Field<Dim, Distance>,
    SdfB: Field<Dim, Uv>,
    Dim: Clone,
{
    fn operator(&self, attr: Uv, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> Vec2 {
        let dist_a = sdf_a.field(Distance, p.clone());
        let dist_b = sdf_b.field(Distance, p.clone());

        if dist_a > dist_b {
            sdf_a.field(attr, p)
        } else {
            sdf_b.field(attr, p)
        }
    }
}

/// Compute the boolean intersection of two distance fields.
pub type Intersection<SdfA, SdfB> = Operator<IntersectionOp, (SdfA, SdfB)>;

impl<SdfA, SdfB> Intersection<SdfA, SdfB> {
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
        prelude::{Cube, Intersection, Point, Sphere},
        test_op_attrs,
    };

    #[test]
    fn test_intersection() {
        Intersection::<Cube, Sphere>::default();
    }

    test_op_attrs!(Intersection::<Point, Point>);
}
