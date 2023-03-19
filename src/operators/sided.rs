//! Given an infinitely-thin surface,
//! divide space into interior and exterior based on axis.

use core::ops::Mul;

use rust_gpu_bridge::prelude::{Dot, Sign, Vec2, Vec3};
use type_fields::Field;

use crate::signed_distance_field::{
    attributes::{distance::Distance, normal::Normal},
    DistanceFunction,
};

use super::{Operator, SignedDistanceOperator};

/// Given an infinitely-thin surface,
/// divide space into interior and exterior based on axis.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
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

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Distance> for SidedOp<Dim>
where
    Sdf: DistanceFunction<Dim, Distance>,
    Dim: Clone + Mul<Dim, Output = Dim> + Sign + Dot,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Distance {
        let mut d = *sdf.evaluate(p.clone());
        d = p.clone().dot(self.axis.clone()).sign();
        d.into()
    }
}

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Normal<Dim>> for SidedOp<Dim>
where
    Sdf: DistanceFunction<Dim, Normal<Dim>>,
    Dim: Clone + Dot + Mul<f32, Output = Dim>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Normal<Dim> {
        ((*sdf.evaluate(p.clone())).clone() * p.dot(self.axis.clone()).sign()).into()
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
