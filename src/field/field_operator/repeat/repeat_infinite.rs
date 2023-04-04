use core::ops::{Add, Mul, Sub};

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Mod,
};
use type_fields::Field;

use crate::prelude::{Attribute, Field, FieldOperator, Operator, Position};

/// Repeat a distance field infinitely in one or more axes.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct RepeatInfiniteOp<Dim> {
    pub period: Dim,
}

impl Default for RepeatInfiniteOp<f32> {
    fn default() -> Self {
        RepeatInfiniteOp { period: 1.0 }
    }
}

impl Default for RepeatInfiniteOp<Vec2> {
    fn default() -> Self {
        RepeatInfiniteOp { period: Vec2::ONE }
    }
}

impl Default for RepeatInfiniteOp<Vec3> {
    fn default() -> Self {
        RepeatInfiniteOp { period: Vec3::ONE }
    }
}

impl<Sdf, Input, Attr> FieldOperator<Sdf, Attr> for RepeatInfiniteOp<Input>
where
    Attr: Attribute<Input = Position<Input>>,
    Sdf: Field<Attr>,
    Input: Add<Input, Output = Input>
        + Add<f32, Output = Input>
        + Sub<Input, Output = Input>
        + Mul<Input, Output = Input>
        + Mul<f32, Output = Input>
        + Mod
        + Clone,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Input>) -> Attr::Output {
        let q = ((**p).clone().add(0.5).mul(self.period.clone()))
            .modulo(self.period.clone())
            .sub(self.period.clone().mul(0.5));
        sdf.field(&q.into())
    }
}

/// Repeat a distance field infinitely in one or more axes.
pub type RepeatInfinite<Dim, Sdf> = Operator<RepeatInfiniteOp<Dim>, Sdf>;

impl<Dim, Sdf> RepeatInfinite<Dim, Sdf> {
    pub fn period(&mut self) -> &mut Dim {
        &mut self.op.period
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use rust_gpu_bridge::glam::{Vec2, Vec3};
    use type_fields::field::Field;

    use crate::{
        prelude::{Point, RepeatInfinite, Sphere},
        test_op_attrs_1d, test_op_attrs_2d, test_op_attrs_3d,
    };

    #[test]
    fn test_repeat_infinite() {
        RepeatInfinite::<_, Sphere>::default().with(RepeatInfinite::period, Vec3::default());
    }

    test_op_attrs_1d!(RepeatInfinite::<f32, Point>);
    test_op_attrs_2d!(RepeatInfinite::<Vec2, Point>);
    test_op_attrs_3d!(RepeatInfinite::<Vec3, Point>);
}
