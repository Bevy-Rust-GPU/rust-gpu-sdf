//! Apply a positional translation to a distance field.

use core::ops::Sub;

use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

/// Apply a positional translation to a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct TranslateOp<Dim> {
    pub translation: Dim,
}

impl<Dim> SignedDistanceOperator<Dim, Distance> for TranslateOp<Dim>
where
    Dim: Clone + Sub<Dim, Output = Dim>,
{
    fn operator<Sdf>(&self, sdf: &Sdf, p: Dim) -> Distance
    where
        Sdf: SignedDistanceField<Dim, Distance>,
    {
        sdf.evaluate(p - self.translation.clone())
    }
}

/// Apply a positional translation to a distance field.
pub type Translate<Sdf, Dim> = Operator<Sdf, TranslateOp<Dim>>;

#[allow(non_camel_case_types)]
pub type Translate_Translation = (crate::operators::Operator_Op, TranslateOp_Translation);

impl<Sdf, Dim> Translate<Sdf, Dim> {
    pub const TRANSLATION: Translate_Translation =
        (Operator::<(), ()>::OP, TranslateOp::<()>::TRANSLATION);
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::Vec3;
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Sphere;

    use super::Translate;

    #[test]
    fn test_translation() {
        Translate::<Sphere, _>::default().with(Translate::<(), ()>::TRANSLATION, Vec3::default());
    }
}
