//! Shift the isosurface of a distance field by a given amount.

use core::ops::Sub;

use rust_gpu_bridge::glam::Vec2;
use type_fields::Field;

use crate::{
    prelude::{Distance, DistanceFunction, Operator, SignedDistanceOperator},
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
    fn operator(&self, attr: Distance, sdf: &Sdf, p: Dim) -> f32 {
        sdf.evaluate(attr, p).sub(self.delta)
    }
}

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Normal<Dim>> for IsosurfaceOp
where
    Sdf: DistanceFunction<Dim, Normal<Dim>>,
{
    fn operator(&self, attr: Normal<Dim>, sdf: &Sdf, p: Dim) -> Dim {
        sdf.evaluate(attr, p)
    }
}

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Uv> for IsosurfaceOp
where
    Sdf: DistanceFunction<Dim, Uv>,
{
    fn operator(&self, attr: Uv, sdf: &Sdf, p: Dim) -> Vec2 {
        sdf.evaluate(attr, p)
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
