//! Convert a solid shape into a hollow one with an infinitely thin surface.

use core::ops::Mul;

use rust_gpu_bridge::{glam::Vec2, Abs, Sign};
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{
        items::position::Position, AttrColor, AttrDistance, AttrNormal, AttrTangent, AttrUv,
        Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv,
    },
};

/// Convert a solid shape into a hollow one with an infinitely thin surface.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct HollowOp;

impl<Sdf, Input> FieldOperator<Sdf, AttrDistance<Input>> for HollowOp
where
    Sdf: Field<AttrDistance<Input>>,
{
    fn operator(&self, sdf: &Sdf, input: &Position<Input>) -> Distance {
        sdf.field(input).abs().into()
    }
}

impl<Sdf, Input> FieldOperator<Sdf, AttrNormal<Input>> for HollowOp
where
    Sdf: Field<AttrDistance<Input>>,
    Sdf: Field<AttrNormal<Input>>,
    Input: Clone + Mul<f32, Output = Input>,
{
    fn operator(&self, sdf: &Sdf, input: &Position<Input>) -> Normal<Input> {
        let d = <Sdf as Field<AttrDistance<Input>>>::field(sdf, input);
        let s = d.sign();
        <Sdf as Field<AttrNormal<Input>>>::field(sdf, &((*input).clone() * s))
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, AttrTangent<Dim>> for HollowOp
where
    Sdf: Field<AttrDistance<Dim>>,
    Sdf: Field<AttrTangent<Dim>>,
    Dim: Clone + Mul<f32, Output = Dim>,
{
    fn operator(&self, sdf: &Sdf, input: &Position<Dim>) -> Tangent<Dim> {
        let d = <Sdf as Field<AttrDistance<Dim>>>::field(sdf, input);
        let s = d.sign();
        <Sdf as Field<AttrTangent<Dim>>>::field(sdf, &((*input).clone() * s))
    }
}

impl<Sdf, Input> FieldOperator<Sdf, AttrUv<Input>> for HollowOp
where
    Sdf: Field<AttrDistance<Input>>,
    Sdf: Field<AttrUv<Input>>,
    Input: Clone + Mul<f32, Output = Input>,
{
    fn operator(&self, sdf: &Sdf, input: &Position<Input>) -> Uv {
        let d = <Sdf as Field<AttrDistance<Input>>>::field(sdf, input);
        let s = d.sign();
        <Sdf as Field<AttrUv<Input>>>::field(sdf, &((*input).clone() * s))
    }
}

impl_passthrough_op_1!(HollowOp, AttrColor<Dim>, Dim);

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
