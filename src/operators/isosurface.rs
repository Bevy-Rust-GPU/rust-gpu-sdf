//! Shift the isosurface of a distance field by a given amount.

use core::ops::Sub;

use type_fields::Field;

use crate::prelude::{Distance, SignedDistanceField, Operator, SignedDistanceOperator};

/// Shift the isosurface of a distance field by a given amount.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct IsosurfaceOp {
    pub delta: f32,
}

impl Default for IsosurfaceOp {
    fn default() -> Self {
        IsosurfaceOp { delta: 1.0 }
    }
}

impl<Dim> SignedDistanceOperator<Dim, Distance> for IsosurfaceOp {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Dim) -> Distance
    where
        Sdf: SignedDistanceField<Dim, Distance>,
    {
        sdf.evaluate(p).sub(self.delta).into()
    }
}

/// Add an arbitrary radius to a distance field.
pub type Isosurface<Sdf> = Operator<Sdf, IsosurfaceOp>;

#[allow(non_camel_case_types)]
pub type Isosurface_Delta = (crate::operators::Operator_Op, IsosurfaceOp_Delta);

impl<Sdf> Isosurface<Sdf> {
    pub const DELTA: Isosurface_Delta = (Operator::<(), ()>::OP, IsosurfaceOp::DELTA);
}

#[cfg(test)]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Point;

    use super::Isosurface;

    #[test]
    fn test_isosurface() {
        Isosurface::<Point>::default().with(Isosurface::<()>::DELTA, f32::default());
    }
}
