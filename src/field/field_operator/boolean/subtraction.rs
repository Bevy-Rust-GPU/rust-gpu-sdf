//! Compute the boolean subtraction of two distance fields.

use core::ops::Neg;

use type_fields::Field;

use crate::prelude::{
    items::position::Position, AttrDistance, AttrNormal, AttrUv, Distance, Field, FieldOperator,
    Normal, Operator, Uv,
};

/// Compute the boolean subtraction of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct SubtractionOp;

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), AttrDistance<Input>> for SubtractionOp
where
    SdfA: Field<AttrDistance<Input>>,
    SdfB: Field<AttrDistance<Input>>,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: &Position<Input>) -> Distance {
        sdf_a.field(p).neg().max(*sdf_b.field(p)).into()
    }
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), AttrNormal<Input>> for SubtractionOp
where
    SdfA: Field<AttrDistance<Input>>,
    SdfA: Field<AttrNormal<Input>>,
    SdfB: Field<AttrDistance<Input>>,
    SdfB: Field<AttrNormal<Input>>,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), input: &Position<Input>) -> Normal<Input> {
        let dist_a = *Field::<AttrDistance<Input>>::field(sdf_a, input);
        let dist_b = *Field::<AttrDistance<Input>>::field(sdf_b, input);

        if -dist_a > dist_b {
            Field::<AttrNormal<Input>>::field(sdf_a, input)
        } else {
            Field::<AttrNormal<Input>>::field(sdf_b, input)
        }
    }
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), AttrUv<Input>> for SubtractionOp
where
    SdfA: Field<AttrDistance<Input>>,
    SdfA: Field<AttrUv<Input>>,
    SdfB: Field<AttrDistance<Input>>,
    SdfB: Field<AttrUv<Input>>,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: &Position<Input>) -> Uv {
        let dist_a = *Field::<AttrDistance<Input>>::field(sdf_a, p);
        let dist_b = *Field::<AttrDistance<Input>>::field(sdf_b, p);

        if -dist_a > dist_b {
            Field::<AttrUv<Input>>::field(sdf_a, p)
        } else {
            Field::<AttrUv<Input>>::field(sdf_b, p)
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
