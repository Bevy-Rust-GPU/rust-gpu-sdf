//! Apply a positional translation to a distance field.

use core::ops::Sub;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Apply a positional translation to a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TranslateOp<Dim> {
    pub translation: Dim,
}

impl<Dim> SignedDistanceOperator<Dim> for TranslateOp<Dim>
where
    Dim: Clone + Sub<Dim, Output = Dim>,
{
    fn operator<Sdf>(&self, sdf: &Sdf, p: Dim) -> f32
    where
        Sdf: SignedDistanceField<Dim>,
    {
        sdf.distance(p - self.translation.clone())
    }
}

/// Apply a positional translation to a distance field.
pub type Translate<Sdf, Dim> = Operator<Sdf, TranslateOp<Dim>, Dim>;
