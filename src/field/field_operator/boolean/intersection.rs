//! Compute the boolean intersection of two distance fields.

use type_fields::macros::Field;

use crate::prelude::{
    items::position::Position, AttrDistance, Field, FieldOperator, AttrNormal, Operator, AttrUv, Distance, Normal, Uv,
};

/// Compute the boolean intersection of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct IntersectionOp;

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), AttrDistance<Input>> for IntersectionOp
where
    SdfA: Field<AttrDistance<Input>>,
    SdfB: Field<AttrDistance<Input>>,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), input: &Position<Input>) -> Distance {
        sdf_a.field(input).max(*sdf_b.field(input)).into()
    }
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), AttrNormal<Input>> for IntersectionOp
where
    SdfA: Field<AttrDistance<Input>>,
    SdfA: Field<AttrNormal<Input>>,
    SdfB: Field<AttrDistance<Input>>,
    SdfB: Field<AttrNormal<Input>>,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), input: &Position<Input>) -> Normal<Input> {
        let dist_a = Field::<AttrDistance<Input>>::field(sdf_a, input);
        let dist_b = Field::<AttrDistance<Input>>::field(sdf_b, input);

        let n = if dist_a > dist_b {
            Field::<AttrNormal<Input>>::field(sdf_a, input)
        } else {
            Field::<AttrNormal<Input>>::field(sdf_b, input)
        };

        n
    }
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), AttrUv<Input>> for IntersectionOp
where
    SdfA: Field<AttrDistance<Input>>,
    SdfA: Field<AttrUv<Input>>,
    SdfB: Field<AttrDistance<Input>>,
    SdfB: Field<AttrUv<Input>>,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), input: &Position<Input>) -> Uv {
        let dist_a = Field::<AttrDistance<Input>>::field(sdf_a, input);
        let dist_b = Field::<AttrDistance<Input>>::field(sdf_b, input);

        if dist_a > dist_b {
            Field::<AttrUv<Input>>::field(sdf_a, input)
        } else {
            Field::<AttrUv<Input>>::field(sdf_b, input)
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
