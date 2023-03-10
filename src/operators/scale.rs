//! Uniformly scale a distance field.

use core::ops::Div;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Uniformly scale a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct ScaleOp {
    scale: f32,
}

impl<Dim> SignedDistanceOperator<Dim> for ScaleOp
where
    Dim: Div<f32, Output = Dim>,
{
    fn operator<Sdf>(&self, sdf: Sdf, p: Dim) -> f32
    where
        Sdf: SignedDistanceField<Dim>,
    {
        sdf.distance(p / self.scale) * self.scale
    }
}

/// Uniformly scale a distance field.
pub type Scale<Sdf, Dim> = Operator<Sdf, ScaleOp, Dim>;
