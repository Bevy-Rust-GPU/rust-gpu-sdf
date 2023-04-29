//! Given an infinitely-thin surface,
//! divide space into interior and exterior based on axis.

use core::ops::Mul;

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Dot, Sign,
};
use type_fields::macros::Field;

use crate::prelude::{
    items::position::Position, AttrDistance, AttrNormal, AttrUv, Distance, Field, Normal, Uv,
};

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

impl<Sdf, Input> FieldOperator<Sdf, AttrDistance<Input>> for SidedOp<Input>
where
    Sdf: Field<AttrDistance<Input>>,
    Input: Clone + Mul<Input, Output = Input> + Sign + Dot,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Input>) -> Distance {
        (*sdf.field(p) * (**p).clone().dot(self.axis.clone()).sign()).into()
    }
}

impl<Sdf, Input> FieldOperator<Sdf, AttrNormal<Input>> for SidedOp<Input>
where
    Sdf: Field<AttrNormal<Input>>,
    Input: Clone + Dot + Mul<f32, Output = Input>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Input>) -> Normal<Input> {
        let n = (*sdf.field(p)).clone();
        (n * (**p).clone().dot(self.axis.clone()).sign()).into()
    }
}

impl<Sdf, Input> FieldOperator<Sdf, AttrUv<Input>> for SidedOp<Input>
where
    Sdf: Field<AttrUv<Input>>,
    Input: Clone + Dot + Mul<f32, Output = Input>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Input>) -> Uv {
        ((*sdf.field(p)).clone() * (**p).clone().dot(self.axis.clone()).sign()).into()
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
