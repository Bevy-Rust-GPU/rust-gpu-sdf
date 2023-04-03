//! Apply a positional translation to a distance field.

use core::ops::Sub;

use type_fields::Field;

use crate::prelude::{Attribute, Field, FieldOperator, Operator};

/// Apply a positional translation to a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct TranslateOp<Dim> {
    pub translation: Dim,
}

impl<Sdf, Dim, Attr> FieldOperator<Sdf, Attr> for TranslateOp<Dim>
where
    Attr: Attribute<Input = Dim>,
    Sdf: Field<Attr>,
    Dim: Clone + Sub<Dim, Output = Dim>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Attr::Output {
        sdf.field(p - self.translation.clone())
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
    use rust_gpu_bridge::glam::{Vec2, Vec3};
    use type_fields::field::Field;

    use crate::{
        prelude::{Point, Sphere, Translate},
        test_op_attrs_1d, test_op_attrs_2d, test_op_attrs_3d,
    };

    #[test]
    fn test_translation() {
        Translate::<_, Sphere>::default().with(Translate::translation, Vec3::default());
    }

    test_op_attrs_1d!(Translate::<f32, Point>);
    test_op_attrs_2d!(Translate::<Vec2, Point>);
    test_op_attrs_3d!(Translate::<Vec3, Point>);
}
