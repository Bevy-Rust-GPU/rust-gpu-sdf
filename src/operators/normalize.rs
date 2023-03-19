use rust_gpu_bridge::prelude::{Sign, Vec2, Vec3};

use crate::signed_distance_field::{
    attributes::{distance::Distance, normal::Normal},
    DistanceFunction,
};

use super::{Operator, SignedDistanceOperator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct NormalizeOp;

impl NormalizeOp {}

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Distance> for NormalizeOp
where
    Sdf: DistanceFunction<Dim, Distance>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Distance {
        sdf.evaluate(p)
    }
}

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Normal<Dim>> for NormalizeOp
where
    Sdf: DistanceFunction<Dim, Normal<Dim>>,
    Dim: Clone + rust_gpu_bridge::prelude::Normalize,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Normal<Dim> {
        Normal((*sdf.evaluate(p)).clone().normalize())
    }
}

pub type Normalize<Sdf> = Operator<NormalizeOp, Sdf>;
