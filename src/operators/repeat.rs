//! Operators for repeating distance fields across a domain.

use core::ops::{Add, Mul, Sub};

use rust_gpu_bridge::{
    modulo::Mod,
    prelude::{Vec2, Vec3},
};
use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

/// Repeat a distance field infinitely in one or more axes.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
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

impl<Dim> SignedDistanceOperator<Dim, Distance> for RepeatInfiniteOp<Dim>
where
    Dim: Add<Dim, Output = Dim>
        + Add<f32, Output = Dim>
        + Sub<Dim, Output = Dim>
        + Mul<Dim, Output = Dim>
        + Mul<f32, Output = Dim>
        + Mod
        + Clone,
{
    fn operator<Sdf>(&self, sdf: &Sdf, p: Dim) -> Distance
    where
        Sdf: SignedDistanceField<Dim, Distance>,
    {
        let q = (p.add(0.5).mul(self.period.clone()))
            .modulo(self.period.clone())
            .sub(self.period.clone().mul(0.5));
        sdf.evaluate(q)
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

impl SignedDistanceOperator<Vec2, Distance> for RepeatCountOp<Vec2> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> Distance
    where
        Sdf: SignedDistanceField<Vec2, Distance>,
    {
        let q = p - self.period * (p / self.period).round().clamp(-self.count, self.count);
        sdf.evaluate(q)
    }
}

impl SignedDistanceOperator<Vec3, Distance> for RepeatCountOp<Vec3> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> Distance
    where
        Sdf: SignedDistanceField<Vec3, Distance>,
    {
        let q = p - self.period * (p / self.period).round().clamp(-self.count, self.count);
        sdf.evaluate(q)
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

#[cfg(test)]
pub mod tests {
    use rust_gpu_bridge::prelude::Vec3;
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Sphere;

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
}
