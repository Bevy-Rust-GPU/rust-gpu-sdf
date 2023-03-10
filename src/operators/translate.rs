//! Apply a positional translation to a distance field.

use rust_gpu_bridge::prelude::Vec3;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Apply a positional translation to a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TranslateOp<P> {
    pub translation: P,
}

impl SignedDistanceOperator<Vec3> for TranslateOp<Vec3> {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        sdf.distance(p - self.translation)
    }
}

/// Apply a positional translation to a distance field.
pub type Translate<Sdf> = Operator<Sdf, TranslateOp<Vec3>, Vec3>;

