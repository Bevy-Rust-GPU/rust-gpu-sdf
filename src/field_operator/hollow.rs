//! Convert a solid shape into a hollow one with an infinitely thin surface.

use core::ops::Mul;

use rust_gpu_bridge::{glam::Vec2, Sign, Abs};
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, FieldFunction, FieldOperator, Normal, Operator, Tangent, Uv},
};

/// Convert a solid shape into a hollow one with an infinitely thin surface.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct HollowOp;

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Distance> for HollowOp
where
    Sdf: FieldFunction<Dim, Distance>,
{
    fn operator(&self, attr: Distance, sdf: &Sdf, p: Dim) -> f32 {
        sdf.evaluate(attr, p).abs()
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Normal<Dim>> for HollowOp
where
    Sdf: FieldFunction<Dim, Distance>,
    Sdf: FieldFunction<Dim, Normal<Dim>>,
    Dim: Clone + Mul<f32, Output = Dim>,
{
    fn operator(&self, attr: Normal<Dim>, sdf: &Sdf, p: Dim) -> Dim {
        let d = sdf.evaluate(Distance, p.clone());
        let s = d.sign();
        sdf.evaluate(attr, p.clone() * s)
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Tangent<Dim>> for HollowOp
where
    Sdf: FieldFunction<Dim, Distance>,
    Sdf: FieldFunction<Dim, Tangent<Dim>>,
    Dim: Clone + Mul<f32, Output = Dim>,
{
    fn operator(&self, attr: Tangent<Dim>, sdf: &Sdf, p: Dim) -> Dim {
        let d = sdf.evaluate(Distance, p.clone());
        let s = d.sign();
        sdf.evaluate(attr, p.clone() * s)
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Uv> for HollowOp
where
    Sdf: FieldFunction<Dim, Distance>,
    Sdf: FieldFunction<Dim, Uv>,
    Dim: Clone + Mul<f32, Output = Dim>,
{
    fn operator(&self, attr: Uv, sdf: &Sdf, p: Dim) -> Vec2 {
        let d = sdf.evaluate(Distance, p.clone());
        let s = d.sign();
        sdf.evaluate(attr, p.clone() * s)
    }
}

impl_passthrough_op_1!(HollowOp, <Dim>, Color);

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
