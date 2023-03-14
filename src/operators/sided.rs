use rust_gpu_bridge::prelude::{Vec2, Vec3};
use type_fields::Field;

use crate::signed_distance_field::attributes::distance::Distance;

use super::{Operator, SignedDistanceOperator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct SidedOp<Dim> {
    pub axis: Dim,
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

pub type Sided<Sdf, Dim> = Operator<Sdf, SidedOp<Dim>>;

impl<Sdf, Dim> Sided<Sdf, Dim> {
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
        Sided::<Line<Vec3>, _>::default().with(Sided::axis, Vec3::default());
    }
}

