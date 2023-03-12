//! Convert a solid shape into a hollow one with an infinitely thin surface.

use rust_gpu_bridge::prelude::{Vec2, Vec3};
use type_fields::Field;

use crate::prelude::{Distance, SignedDistanceField, Operator, SignedDistanceOperator};

/// Convert a solid shape into a hollow one with an infinitely thin surface.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct HollowOp;

impl SignedDistanceOperator<Vec2, Distance> for HollowOp {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> Distance
    where
        Sdf: SignedDistanceField<Vec2, Distance>,
    {
        sdf.evaluate(p).abs().into()
    }
}

impl SignedDistanceOperator<Vec3, Distance> for HollowOp {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> Distance
    where
        Sdf: SignedDistanceField<Vec3, Distance>,
    {
        sdf.evaluate(p).abs().into()
    }
}

/// Convert a solid shape into a hollow one with an infinitely thin surface.
pub type Hollow<Sdf> = Operator<Sdf, HollowOp>;
