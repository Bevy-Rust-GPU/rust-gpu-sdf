//! Uniformly scale a distance field.

use core::ops::{Div, Mul};

use type_fields::Field;

use crate::prelude::{Distance, Operator, DistanceFunction, SignedDistanceOperator};

/// Uniformly scale a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct ScaleOp {
    pub scale: f32,
}

impl<Sdf, Dim, Out> SignedDistanceOperator<Sdf, Dim, Out> for ScaleOp
where
    Sdf: DistanceFunction<Dim, Out>,
    Dim: Div<f32, Output = Dim>,
    Out: Mul<f32, Output = Out>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Out {
        sdf.evaluate(p / self.scale).mul(self.scale).into()
    }
}

/// Uniformly scale a distance field.
pub type Scale<Sdf> = Operator<ScaleOp, Sdf>;

impl<Sdf> Scale<Sdf> {
    pub fn scale(&mut self) -> &mut f32 {
        &mut self.op.scale
    }
}

#[cfg(test)]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Cube;

    use super::Scale;

    #[test]
    fn test_scale() {
        Scale::<Cube>::default().with(Scale::scale, f32::default());
    }
}
