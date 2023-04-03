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

impl<Sdf, Dim> FieldOperator<Sdf, Distance<Dim>> for HollowOp
where
    Sdf: Field<Distance<Dim>>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> f32 {
        sdf.field(p).abs()
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Normal<Dim>> for HollowOp
where
    Sdf: Field<Distance<Dim>>,
    Sdf: Field<Normal<Dim>>,
    Dim: Clone + Mul<f32, Output = Dim>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Dim {
        let d = <Sdf as Field<Distance<Dim>>>::field(sdf, p.clone());
        let s = d.sign();
        <Sdf as Field<Normal<Dim>>>::field(sdf, p.clone() * s)
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Tangent<Dim>> for HollowOp
where
    Sdf: Field<Distance<Dim>>,
    Sdf: Field<Tangent<Dim>>,
    Dim: Clone + Mul<f32, Output = Dim>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Dim {
        let d = <Sdf as Field<Distance<Dim>>>::field(sdf, p.clone());
        let s = d.sign();
        <Sdf as Field<Tangent<Dim>>>::field(sdf, p.clone() * s)
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Uv<Dim>> for HollowOp
where
    Sdf: Field<Distance<Dim>>,
    Sdf: Field<Uv<Dim>>,
    Dim: Clone + Mul<f32, Output = Dim>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Vec2 {
        let d = <Sdf as Field<Distance<Dim>>>::field(sdf, p.clone());
        let s = d.sign();
        <Sdf as Field<Uv<Dim>>>::field(sdf, p.clone() * s)
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
