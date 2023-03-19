//! Shift the isosurface of a distance field by a given amount.

use core::ops::Sub;

use type_fields::Field;

use crate::{
    prelude::{Distance, Operator, DistanceFunction, SignedDistanceOperator},
    signed_distance_field::attributes::{normal::Normal, uv::Uv},
};

/// Shift the isosurface of a distance field by a given amount.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct IsosurfaceOp {
    pub delta: f32,
}

impl Default for IsosurfaceOp {
    fn default() -> Self {
        IsosurfaceOp { delta: 1.0 }
    }
}

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Distance> for IsosurfaceOp
where
    Sdf: DistanceFunction<Dim, Distance>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Distance {
        sdf.evaluate(p).sub(self.delta).into()
    }
}

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Normal<Dim>> for IsosurfaceOp
where
    Sdf: DistanceFunction<Dim, Normal<Dim>>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Normal<Dim> {
        sdf.evaluate(p)
    }
}

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Uv> for IsosurfaceOp
where
    Sdf: DistanceFunction<Dim, Uv>,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Uv {
        sdf.evaluate(p)
    }
}

/// Add an arbitrary radius to a distance field.
pub type Isosurface<Sdf> = Operator<IsosurfaceOp, Sdf>;

impl<Sdf> Isosurface<Sdf> {
    pub fn delta(&mut self) -> &mut f32 {
        &mut self.op.delta
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Point;

    use super::Isosurface;

    #[test]
    fn test_isosurface() {
        Isosurface::<Point>::default().with(Isosurface::delta, f32::default());
    }
}
