//! Convert a solid shape into a hollow one with an infinitely thin surface.

use core::ops::Mul;

use rust_gpu_bridge::{Abs, Sign};
use type_fields::Field;

use crate::prelude::{Distance, FieldFunction, FieldOperator, Normal, Operator};

/// Convert a solid shape into a hollow one with an infinitely thin surface.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct HollowOp;

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Distance> for HollowOp
where
    Sdf: FieldFunction<Dim, Distance>,
{
    fn operator(&self, attr: Distance, sdf: &Sdf, p: Dim) -> f32 {
        sdf.evaluate(attr, p).abs()
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Normal<Dim>> for HollowOp
where
    Sdf: FieldFunction<Dim, Distance>,
    Sdf: FieldFunction<Dim, Normal<Dim>>,
    Dim: Clone + Mul<f32, Output = Dim>,
{
    fn operator(&self, attr: Normal<Dim>, sdf: &Sdf, p: Dim) -> Dim {
        let d = sdf.evaluate(Distance, p.clone());
        let s = d.sign();
        sdf.evaluate(attr, p.clone() * s)
    }
}

/// Convert a solid shape into a hollow one with an infinitely thin surface.
pub type Hollow<Sdf> = Operator<Sdf, HollowOp>;
