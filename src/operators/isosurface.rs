//! Shift the isosurface of a distance field by a given amount.

use core::ops::Sub;

use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

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

impl<Sdf> Isosurface<Sdf> {
    pub fn delta(&mut self) -> &mut f32 {
        &mut self.op.delta
    }
}

#[cfg(test)]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Point;

    use super::Isosurface;

    #[test]
    fn test_isosurface() {
        Isosurface::<Point>::default().with(Isosurface::delta, f32::default());
    }
}
