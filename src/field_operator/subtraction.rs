//! Compute the boolean subtraction of two distance fields.

use core::ops::Neg;

use rust_gpu_bridge::glam::Vec2;
use type_fields::Field;

use crate::prelude::{Distance, FieldFunction, FieldOperator, Normal, Operator, Uv};

/// Compute the boolean subtraction of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct SubtractionOp;

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Distance> for SubtractionOp
where
    SdfA: FieldFunction<Dim, Distance>,
    SdfB: FieldFunction<Dim, Distance>,
    Dim: Clone,
{
    fn operator(&self, attr: Distance, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> f32 {
        sdf_a
            .field(attr, p.clone())
            .neg()
            .max(sdf_b.field(attr, p))
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Normal<Dim>> for SubtractionOp
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

        if -dist_a > dist_b {
            sdf_a.field(attr, p)
        } else {
            sdf_b.field(attr, p)
        }
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Uv> for SubtractionOp
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

        if -dist_a > dist_b {
            sdf_a.field(attr, p)
        } else {
            sdf_b.field(attr, p)
        }
    }
}

/// Compute the boolean subtraction of two distance fields.
pub type Subtraction<SdfA, SdfB> = Operator<SubtractionOp, (SdfA, SdfB)>;

impl<SdfA, SdfB> Subtraction<SdfA, SdfB> {
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
        prelude::{Cube, Point, Sphere, Subtraction},
        test_op_attrs,
    };

    #[test]
    fn test_subtraction() {
        Subtraction::<Cube, Sphere>::default();
    }

    test_op_attrs!(Subtraction::<Point, Point>);
}
