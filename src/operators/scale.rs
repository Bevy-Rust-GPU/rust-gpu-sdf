//! Uniformly scale a distance field.

use core::ops::{Div, Mul};

use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

/// Uniformly scale a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct ScaleOp {
    pub scale: f32,
}

impl<Dim> SignedDistanceOperator<Dim, Distance> for ScaleOp
where
    Dim: Div<f32, Output = Dim>,
{
    fn operator<Sdf>(&self, sdf: &Sdf, p: Dim) -> Distance
    where
        Sdf: SignedDistanceField<Dim, Distance>,
    {
        sdf.evaluate(p / self.scale).mul(self.scale).into()
    }
}

/// Uniformly scale a distance field.
pub type Scale<Sdf> = Operator<Sdf, ScaleOp>;

#[allow(non_camel_case_types)]
pub type Scale_Scale = (crate::operators::Operator_Op, ScaleOp_Scale);

impl<Sdf> Scale<Sdf> {
    pub const SCALE: Scale_Scale = (Operator::<(), ()>::OP, ScaleOp::SCALE);
}

#[cfg(test)]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Cube;

    use super::Scale;

    #[test]
    fn test_scale() {
        Scale::<Cube>::default().with(Scale::<()>::SCALE, f32::default());
    }
}
