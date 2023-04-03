//! Given an infinitely-thin surface,
//! divide space into interior and exterior based on axis.

use core::ops::Mul;

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Dot, Sign,
};
use type_fields::Field;

use crate::prelude::{Distance, Field, Normal, Uv};

use super::{FieldOperator, Operator};

/// Given an infinitely-thin surface,
/// divide space into interior and exterior based on axis.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct SidedOp<Dim> {
    pub axis: Dim,
}

impl Default for SidedOp<f32> {
    fn default() -> Self {
        SidedOp { axis: 1.0 }
    }
}

impl Default for SidedOp<Vec2> {
    fn default() -> Self {
        SidedOp { axis: Vec2::Y }
    }
}

impl Default for SidedOp<Vec3> {
    fn default() -> Self {
        SidedOp { axis: Vec3::Y }
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Distance<Dim>> for SidedOp<Dim>
where
    Sdf: Field<Distance<Dim>>,
    Dim: Clone + Mul<Dim, Output = Dim> + Sign + Dot,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> f32 {
        sdf.field(p.clone()) * p.clone().dot(self.axis.clone()).sign()
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Normal<Dim>> for SidedOp<Dim>
where
    Sdf: Field<Normal<Dim>>,
    Dim: Clone + Dot + Mul<f32, Output = Dim>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Dim {
        (sdf.field(p.clone())).clone() * p.dot(self.axis.clone()).sign()
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Uv<Dim>> for SidedOp<Dim>
where
    Sdf: Field<Uv<Dim>>,
    Dim: Clone + Dot + Mul<f32, Output = Dim>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Vec2 {
        (sdf.field(p.clone())).clone() * p.dot(self.axis.clone()).sign()
    }
}

pub type Sided<Dim, Sdf> = Operator<SidedOp<Dim>, Sdf>;

/// Given an infinitely-thin surface,
/// divide space into interior and exterior based on axis.
impl<Dim, Sdf> Sided<Dim, Sdf> {
    pub fn axis(&mut self) -> &mut Dim {
        &mut self.op.axis
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::glam::{Vec2, Vec3};
    use type_fields::field::Field;

    use crate::{
        prelude::{Line, Point, Sided},
        test_op_attrs_1d, test_op_attrs_2d, test_op_attrs_3d,
    };

    #[test]
    fn test_sided() {
        Sided::<_, Line<Vec3>>::default().with(Sided::axis, Vec3::default());
    }

    test_op_attrs_1d!(Sided::<f32, Point>);
    test_op_attrs_2d!(Sided::<Vec2, Point>);
    test_op_attrs_3d!(Sided::<Vec3, Point>);
}
