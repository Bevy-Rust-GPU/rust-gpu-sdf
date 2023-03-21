//! Convert a solid shape into a hollow one with an infinitely thin surface.

use core::ops::Mul;

use rust_gpu_bridge::{Abs, Sign};
use type_fields::Field;

use crate::{
    prelude::{Distance, Operator, DistanceFunction, SignedDistanceOperator},
    signed_distance_field::attributes::normal::Normal,
};

/// Convert a solid shape into a hollow one with an infinitely thin surface.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct HollowOp;

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Distance> for HollowOp
where
    Sdf: DistanceFunction<Dim, Distance>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Distance {
        sdf.evaluate(p).abs().into()
    }
}

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Normal<Dim>> for HollowOp
where
    Sdf: DistanceFunction<Dim, Distance>,
    Sdf: DistanceFunction<Dim, Normal<Dim>>,
    Dim: Clone + Mul<f32, Output = Dim>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Normal<Dim> {
        let d: Distance = sdf.evaluate(p.clone());
        let s = d.sign();
        let n: Normal<Dim> = sdf.evaluate(p.clone() * s);
        n.into()
    }
}

/// Convert a solid shape into a hollow one with an infinitely thin surface.
pub type Hollow<Sdf> = Operator<Sdf, HollowOp>;
