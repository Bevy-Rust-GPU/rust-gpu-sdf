//! Compute the boolean intersection of two distance fields.

use rust_gpu_bridge::glam::Vec2;
use type_fields::Field;

use crate::prelude::{Distance, Field, FieldOperator, Normal, Operator, Uv};

/// Compute the boolean intersection of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct IntersectionOp;

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), Distance<Input>> for IntersectionOp
where
    SdfA: Field<Distance<Input>>,
    SdfB: Field<Distance<Input>>,
    Input: Clone,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: &Input) -> f32 {
        sdf_a.field(p).max(sdf_b.field(p))
    }
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), Normal<Input>> for IntersectionOp
where
    SdfA: Field<Distance<Input>>,
    SdfA: Field<Normal<Input>>,
    SdfB: Field<Distance<Input>>,
    SdfB: Field<Normal<Input>>,
    Input: Clone,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: &Input) -> Input {
        let dist_a = Field::<Distance<Input>>::field(sdf_a, p);
        let dist_b = Field::<Distance<Input>>::field(sdf_b, p);

        let n = if dist_a > dist_b {
            Field::<Normal<Input>>::field(sdf_a, p)
        } else {
            Field::<Normal<Input>>::field(sdf_b, p)
        };

        n.into()
    }
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), Uv<Input>> for IntersectionOp
where
    SdfA: Field<Distance<Input>>,
    SdfA: Field<Uv<Input>>,
    SdfB: Field<Distance<Input>>,
    SdfB: Field<Uv<Input>>,
    Input: Clone,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), input: &Input) -> Vec2 {
        let dist_a = Field::<Distance<Input>>::field(sdf_a, input);
        let dist_b = Field::<Distance<Input>>::field(sdf_b, input);

        if dist_a > dist_b {
            Field::<Uv<Input>>::field(sdf_a, input)
        } else {
            Field::<Uv<Input>>::field(sdf_b, input)
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
