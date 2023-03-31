//! Operators for repeating distance fields across a domain.

use core::ops::{Add, Div, Mul, Neg, Sub};

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Clamp, Mod, Round,
};
use type_fields::Field;

use crate::prelude::{Attribute, Field, FieldOperator, Operator};

/// Repeat a distance field infinitely in one or more axes.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
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

impl<Sdf, Dim, Attr> FieldOperator<Sdf, Dim, Attr> for RepeatInfiniteOp<Dim>
where
    Attr: Attribute,
    Sdf: Field<Dim, Attr>,
    Dim: Add<Dim, Output = Dim>
        + Add<f32, Output = Dim>
        + Sub<Dim, Output = Dim>
        + Mul<Dim, Output = Dim>
        + Mul<f32, Output = Dim>
        + Mod
        + Clone,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: Dim) -> Attr::Type {
        let q = (p.add(0.5).mul(self.period.clone()))
            .modulo(self.period.clone())
            .sub(self.period.clone().mul(0.5));
        sdf.field(attr, q)
    }
}

/// Repeat a distance field infinitely in one or more axes.
pub type RepeatInfinite<Dim, Sdf> = Operator<RepeatInfiniteOp<Dim>, Sdf>;

impl<Dim, Sdf> RepeatInfinite<Dim, Sdf> {
    pub fn period(&mut self) -> &mut Dim {
        &mut self.op.period
    }
}

/// Repeat a distance field a set number of times in one or more axes.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct RepeatCountOp<Dim> {
    pub period: Dim,
    pub count: Dim,
}

impl Default for RepeatCountOp<Vec2> {
    fn default() -> Self {
        RepeatCountOp {
            period: Vec2::ONE,
            count: Vec2::ONE * 1.0,
        }
    }
}

impl Default for RepeatCountOp<Vec3> {
    fn default() -> Self {
        RepeatCountOp {
            period: Vec3::ONE,
            count: Vec3::ONE * 1.0,
        }
    }
}

impl<Sdf, Dim, Attr> FieldOperator<Sdf, Dim, Attr> for RepeatCountOp<Dim>
where
    Attr: Attribute,
    Sdf: Field<Dim, Attr>,
    Dim: Clone
        + Div<Dim, Output = Dim>
        + Neg<Output = Dim>
        + Mul<Dim, Output = Dim>
        + Sub<Dim, Output = Dim>
        + Round
        + Clamp,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: Dim) -> Attr::Type {
        let q = p.clone()
            - self.period.clone()
                * (p / self.period.clone())
                    .round()
                    .clamp(-self.count.clone(), self.count.clone());
        sdf.field(attr, q)
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
        prelude::{Point, Sphere},
        test_op_attrs_1d, test_op_attrs_2d, test_op_attrs_3d,
    };

    use super::{RepeatCount, RepeatInfinite};

    #[test]
    fn test_repeat_infinite() {
        RepeatInfinite::<_, Sphere>::default().with(RepeatInfinite::period, Vec3::default());
    }

    #[test]
    fn test_repeat_count() {
        RepeatCount::<_, Sphere>::default()
            .with(RepeatCount::period, Vec3::default())
            .with(RepeatCount::count, Vec3::default());
    }

    test_op_attrs_1d!(RepeatInfinite::<f32, Point>);
    test_op_attrs_2d!(RepeatInfinite::<Vec2, Point>);
    test_op_attrs_3d!(RepeatInfinite::<Vec3, Point>);
}
