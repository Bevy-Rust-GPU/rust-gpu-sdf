//! Given an infinitely-thin surface,
//! divide space into interior and exterior based on axis.

use rust_gpu_bridge::prelude::{Vec2, Vec3};
use type_fields::Field;

use crate::signed_distance_field::attributes::distance::Distance;

use super::{Operator, SignedDistanceOperator};

/// Given an infinitely-thin surface,
/// divide space into interior and exterior based on axis.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
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

impl SignedDistanceOperator<f32, Distance> for SidedOp<f32> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: f32) -> Distance
    where
        Sdf: crate::signed_distance_field::SignedDistanceField<f32, Distance>,
    {
        let mut d = *sdf.evaluate(p);
        d *= (p * self.axis).signum();
        d.into()
    }
}

impl SignedDistanceOperator<Vec2, Distance> for SidedOp<Vec2> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> Distance
    where
        Sdf: crate::signed_distance_field::SignedDistanceField<Vec2, Distance>,
    {
        let mut d = *sdf.evaluate(p);
        d *= p.dot(self.axis).signum();
        d.into()
    }
}

impl SignedDistanceOperator<Vec3, Distance> for SidedOp<Vec3> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> Distance
    where
        Sdf: crate::signed_distance_field::SignedDistanceField<Vec3, Distance>,
    {
        let mut d = *sdf.evaluate(p);
        d *= p.dot(self.axis).signum();
        d.into()
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

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::Vec3;
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Line;

    use super::Sided;

    #[test]
    fn test_sided() {
        Sided::<_, Line<Vec3>>::default().with(Sided::axis, Vec3::default());
    }
}
