//! Operators for repeating distance fields across a domain.

use core::ops::{Div, Mul, Neg, Sub};

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Clamp, Round,
};
use type_fields::Field;

use crate::prelude::{Attribute, Field, FieldOperator, Operator};

/// Repeat a distance field a set number of times in one or more axes.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct RepeatCountOp<Dim> {
    pub period: Dim,
    pub count: Dim,
}

impl Default for RepeatCountOp<f32> {
    fn default() -> Self {
        RepeatCountOp {
            period: 1.0,
            count: 1.0,
        }
    }
}

impl Default for RepeatCountOp<Vec2> {
    fn default() -> Self {
        RepeatCountOp {
            period: Vec2::ONE,
            count: Vec2::ONE,
        }
    }
}

impl Default for RepeatCountOp<Vec3> {
    fn default() -> Self {
        RepeatCountOp {
            period: Vec3::ONE,
            count: Vec3::ONE,
        }
    }
}

impl<Sdf, Input, Attr> FieldOperator<Sdf, Attr> for RepeatCountOp<Input>
where
    Attr: Attribute<Input = Input>,
    Sdf: Field<Attr>,
    Input: Clone
        + Div<Input, Output = Input>
        + Neg<Output = Input>
        + Mul<Input, Output = Input>
        + Sub<Input, Output = Input>
        + Round
        + Clamp,
{
    fn operator(&self, sdf: &Sdf, p: &Input) -> Attr::Output {
        let q = p.clone()
            - self.period.clone()
                * (p.clone() / self.period.clone())
                    .round()
                    .clamp(-self.count.clone(), self.count.clone());
        sdf.field(&q)
    }
}

/// Repeat a distance field a set number of times in one or more axes.
pub type RepeatCount<Dim, Sdf> = Operator<RepeatCountOp<Dim>, Sdf>;

impl<Dim, Sdf> RepeatCount<Dim, Sdf> {
    pub fn period(&mut self) -> &mut Dim {
        &mut self.op.period
    }

    pub fn count(&mut self) -> &mut Dim {
        &mut self.op.count
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use rust_gpu_bridge::glam::{Vec2, Vec3};
    use type_fields::field::Field;

    use crate::{
        prelude::{Point, RepeatCount, Sphere},
        test_op_attrs_1d, test_op_attrs_2d, test_op_attrs_3d,
    };

    #[test]
    fn test_repeat_count() {
        RepeatCount::<_, Sphere>::default()
            .with(RepeatCount::period, Vec3::default())
            .with(RepeatCount::count, Vec3::default());
    }

    test_op_attrs_1d!(RepeatCount::<f32, Point>);
    test_op_attrs_2d!(RepeatCount::<Vec2, Point>);
    test_op_attrs_3d!(RepeatCount::<Vec3, Point>);
}
