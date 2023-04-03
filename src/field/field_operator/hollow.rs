//! Convert a solid shape into a hollow one with an infinitely thin surface.

use core::ops::Mul;

use rust_gpu_bridge::{glam::Vec2, Abs, Sign};
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv},
};

/// Convert a solid shape into a hollow one with an infinitely thin surface.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct HollowOp;

impl<Sdf, Input> FieldOperator<Sdf, Distance<Input>> for HollowOp
where
    Sdf: Field<Distance<Input>>,
{
    fn operator(&self, sdf: &Sdf, input: &Input) -> f32 {
        sdf.field(input).abs()
    }
}

impl<Sdf, Input> FieldOperator<Sdf, Normal<Input>> for HollowOp
where
    Sdf: Field<Distance<Input>>,
    Sdf: Field<Normal<Input>>,
    Input: Clone + Mul<f32, Output = Input>,
{
    fn operator(&self, sdf: &Sdf, input: &Input) -> Input {
        let d = <Sdf as Field<Distance<Input>>>::field(sdf, input);
        let s = d.sign();
        <Sdf as Field<Normal<Input>>>::field(sdf, &(input.clone() * s))
    }
}

impl<Sdf, Input> FieldOperator<Sdf, Tangent<Input>> for HollowOp
where
    Sdf: Field<Distance<Input>>,
    Sdf: Field<Tangent<Input>>,
    Input: Clone + Mul<f32, Output = Input>,
{
    fn operator(&self, sdf: &Sdf, input: &Input) -> Input {
        let d = <Sdf as Field<Distance<Input>>>::field(sdf, input);
        let s = d.sign();
        <Sdf as Field<Tangent<Input>>>::field(sdf, &(input.clone() * s))
    }
}

impl<Sdf, Input> FieldOperator<Sdf, Uv<Input>> for HollowOp
where
    Sdf: Field<Distance<Input>>,
    Sdf: Field<Uv<Input>>,
    Input: Clone + Mul<f32, Output = Input>,
{
    fn operator(&self, sdf: &Sdf, input: &Input) -> Vec2 {
        let d = <Sdf as Field<Distance<Input>>>::field(sdf, input);
        let s = d.sign();
        <Sdf as Field<Uv<Input>>>::field(sdf, &(input.clone() * s))
    }
}

impl_passthrough_op_1!(HollowOp, Color<Dim>, Dim);

/// Convert a solid shape into a hollow one with an infinitely thin surface.
pub type Hollow<Sdf> = Operator<HollowOp, Sdf>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use crate::{prelude::Point, test_op_attrs};

    use super::Hollow;

    #[test]
    fn test_gradient_tetrahedron() {
        Hollow::<Point>::default();
    }

    test_op_attrs!(Hollow::<Point>);
}
