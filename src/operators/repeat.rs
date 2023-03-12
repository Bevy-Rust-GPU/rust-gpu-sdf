//! Operators for repeating distance fields across a domain.

use rust_gpu_bridge::{
    modulo::Mod,
    prelude::{Vec2, Vec3},
};
use type_fields::Field;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

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

impl SignedDistanceOperator<Vec2> for RepeatInfiniteOp<Vec2> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> f32
    where
        Sdf: SignedDistanceField<Vec2>,
    {
        let q = (p + 0.5 * self.period).modulo(self.period) - (0.5 * self.period);
        sdf.distance(q)
    }
}

impl SignedDistanceOperator<Vec3> for RepeatInfiniteOp<Vec3> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        let q = (p + 0.5 * self.period).modulo(self.period) - (0.5 * self.period);
        sdf.distance(q)
    }
}

/// Repeat a distance field infinitely in one or more axes.
pub type RepeatInfinite<Sdf, Dim> = Operator<Sdf, RepeatInfiniteOp<Dim>>;

#[allow(non_camel_case_types)]
pub type RepeatInfinite_Period = (crate::operators::Operator_Op, RepeatInfiniteOp_Period);

impl<Sdf, Dim> RepeatInfinite<Sdf, Dim> {
    pub const PERIOD: RepeatInfinite_Period =
        (Operator::<(), ()>::OP, RepeatInfiniteOp::<()>::PERIOD);
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

impl SignedDistanceOperator<Vec2> for RepeatCountOp<Vec2> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> f32
    where
        Sdf: SignedDistanceField<Vec2>,
    {
        let q = p - self.period * (p / self.period).round().clamp(-self.count, self.count);
        sdf.distance(q)
    }
}

impl SignedDistanceOperator<Vec3> for RepeatCountOp<Vec3> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        let q = p - self.period * (p / self.period).round().clamp(-self.count, self.count);
        sdf.distance(q)
    }
}

/// Repeat a distance field a set number of times in one or more axes.
pub type RepeatCount<Sdf, Dim> = Operator<Sdf, RepeatCountOp<Dim>>;

#[allow(non_camel_case_types)]
pub type RepeatCount_Period = (crate::operators::Operator_Op, RepeatCountOp_Period);

#[allow(non_camel_case_types)]
pub type RepeatCount_Count = (crate::operators::Operator_Op, RepeatCountOp_Count);

impl<Sdf, Dim> RepeatCount<Sdf, Dim> {
    pub const PERIOD: RepeatCount_Period = (Operator::<(), ()>::OP, RepeatCountOp::<()>::PERIOD);
    pub const COUNT: RepeatCount_Count = (Operator::<(), ()>::OP, RepeatCountOp::<()>::COUNT);
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
            .with(RepeatInfinite::<(), ()>::PERIOD, Vec3::default());
    }

    #[test]
    fn test_repeat_count() {
        RepeatCount::<Sphere, _>::default()
            .with(RepeatCount::<(), ()>::PERIOD, Vec3::default())
            .with(RepeatCount::<(), ()>::COUNT, Vec3::default());
    }
}
