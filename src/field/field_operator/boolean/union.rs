//! Compute the boolean union of two distance fields.

use rust_gpu_bridge::glam::Vec2;
use type_fields::Field;

use crate::prelude::{Distance, Field, FieldOperator, Normal, Operator, Uv};

/// Compute the boolean union of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct UnionOp;

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Distance<Dim>> for UnionOp
where
    SdfA: Field<Distance<Dim>>,
    SdfB: Field<Distance<Dim>>,
    Dim: Clone,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> f32 {
        sdf_a.field(p.clone()).min(sdf_b.field(p))
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Normal<Dim>> for UnionOp
where
    SdfA: Field<Distance<Dim>>,
    SdfA: Field<Normal<Dim>>,
    SdfB: Field<Distance<Dim>>,
    SdfB: Field<Normal<Dim>>,
    Dim: Clone,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> Dim {
        let dist_a = Field::<Distance<Dim>>::field(sdf_a, p.clone());
        let dist_b = Field::<Distance<Dim>>::field(sdf_b, p.clone());

        if dist_a < dist_b {
            Field::<Normal<Dim>>::field(sdf_a, p)
        } else {
            Field::<Normal<Dim>>::field(sdf_b, p)
        }
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Uv<Dim>> for UnionOp
where
    SdfA: Field<Distance<Dim>>,
    SdfA: Field<Uv<Dim>>,
    SdfB: Field<Distance<Dim>>,
    SdfB: Field<Uv<Dim>>,
    Dim: Clone,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> Vec2 {
        let dist_a = Field::<Distance<Dim>>::field(sdf_a, p.clone());
        let dist_b = Field::<Distance<Dim>>::field(sdf_b, p.clone());

        if dist_a < dist_b {
            Field::<Uv<Dim>>::field(sdf_a, p)
        } else {
            Field::<Uv<Dim>>::field(sdf_a, p)
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
