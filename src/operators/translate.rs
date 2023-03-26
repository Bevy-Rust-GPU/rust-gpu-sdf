//! Apply a positional translation to a distance field.

use core::ops::Sub;

use type_fields::Field;

use crate::{
    prelude::{DistanceFunction, Operator, SignedDistanceOperator},
    signed_distance_field::attributes::Attribute,
};

/// Apply a positional translation to a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct TranslateOp<Dim> {
    pub translation: Dim,
}

impl<Sdf, Dim, Attr> SignedDistanceOperator<Sdf, Dim, Attr> for TranslateOp<Dim>
where
    Attr: Attribute,
    Sdf: DistanceFunction<Dim, Attr>,
    Dim: Clone + Sub<Dim, Output = Dim>,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: Dim) -> Attr::Type {
        sdf.evaluate(attr, p - self.translation.clone())
    }
}

/// Apply a positional translation to a distance field.
pub type Translate<Dim, Sdf> = Operator<TranslateOp<Dim>, Sdf>;

impl<Dim, Sdf> Translate<Dim, Sdf> {
    pub fn translation(&mut self) -> &mut Dim {
        &mut self.op.translation
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::glam::Vec3;
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Sphere;

    use super::Translate;

    #[test]
    fn test_translation() {
        Translate::<_, Sphere>::default().with(Translate::translation, Vec3::default());
    }
}
