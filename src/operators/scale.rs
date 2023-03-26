//! Uniformly scale a distance field.

use core::ops::{Div, Mul};

use type_fields::Field;

use crate::{
    prelude::{FieldFunction, Operator, SignedDistanceOperator},
    signed_distance_field::attributes::Attribute,
};

/// Uniformly scale a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct ScaleOp {
    pub scale: f32,
}

impl<Sdf, Dim, Attr> SignedDistanceOperator<Sdf, Dim, Attr> for ScaleOp
where
    Attr: Attribute,
    Sdf: FieldFunction<Dim, Attr>,
    Dim: Div<f32, Output = Dim>,
    Attr::Type: Mul<f32, Output = Attr::Type>,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: Dim) -> Attr::Type {
        sdf.evaluate(attr, p / self.scale).mul(self.scale)
    }
}

/// Uniformly scale a distance field.
pub type Scale<Sdf> = Operator<ScaleOp, Sdf>;

impl<Sdf> Scale<Sdf> {
    pub fn scale(&mut self) -> &mut f32 {
        &mut self.op.scale
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Cube;

    use super::Scale;

    #[test]
    fn test_scale() {
        Scale::<Cube>::default().with(Scale::scale, f32::default());
    }
}
