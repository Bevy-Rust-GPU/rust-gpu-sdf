//! Operators for repeating distance fields across a domain.

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

impl SignedDistanceOperator<Vec2, Distance> for RepeatInfiniteOp<Vec2> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> Distance
    where
        Sdf: SignedDistanceField<Vec2, Distance>,
    {
        let q = (p + 0.5 * self.period).modulo(self.period) - (0.5 * self.period);
        sdf.evaluate(q)
    }
}

impl SignedDistanceOperator<Vec3, Distance> for RepeatInfiniteOp<Vec3> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> Distance
    where
        Sdf: SignedDistanceField<Vec3, Distance>,
    {
        let q = (p + 0.5 * self.period).modulo(self.period) - (0.5 * self.period);
        sdf.evaluate(q)
    }
}

/// Repeat a distance field infinitely in one or more axes.
pub type RepeatInfinite<Sdf, Dim> = Operator<Sdf, RepeatInfiniteOp<Dim>>;

impl<Sdf, Dim> RepeatInfinite<Sdf, Dim> {
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
pub type RepeatCount<Sdf, Dim> = Operator<Sdf, RepeatCountOp<Dim>>;

impl<Sdf, Dim> RepeatCount<Sdf, Dim> {
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
        RepeatInfinite::<Sphere, _>::default()
            .with(RepeatInfinite::period, Vec3::default());
    }

    #[test]
    fn test_repeat_count() {
        RepeatCount::<Sphere, _>::default()
            .with(RepeatCount::period, Vec3::default())
            .with(RepeatCount::count, Vec3::default());
    }
}
