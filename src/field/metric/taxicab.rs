//! Taxicab distance metric.

use core::ops::Add;

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs, Normalize, Sign,
};

use crate::prelude::{
    items::position::Position, AttrDistance, AttrNormal, Distance, Field, Normal,
};

/// Taxicab distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct TaxicabMetric;

impl Field<AttrDistance<f32>> for TaxicabMetric {
    fn field(&self, p: &Position<f32>) -> Distance {
        p.abs().into()
    }
}

impl Field<AttrDistance<Vec2>> for TaxicabMetric {
    fn field(&self, p: &Position<Vec2>) -> Distance {
        p.x.abs().add(p.y.abs()).into()
    }
}

impl Field<AttrDistance<Vec3>> for TaxicabMetric {
    fn field(&self, p: &Position<Vec3>) -> Distance {
        p.x.abs().add(p.y.abs()).add(p.z.abs()).into()
    }
}

impl<Input> Field<AttrNormal<Input>> for TaxicabMetric
where
    Input: Clone + Sign + Normalize,
{
    fn field(&self, input: &Position<Input>) -> Normal<Input> {
        (**input).clone().sign().normalize().into()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::prelude::{BoundTester, TaxicabMetric};

    #[test]
    #[should_panic]
    pub fn test_taxicab_metric_2d() {
        assert!(BoundTester::<TaxicabMetric>::default().is_field_2d());
    }

    #[test]
    #[should_panic]
    pub fn test_taxicab_metric_3d() {
        assert!(BoundTester::<TaxicabMetric>::default().is_field_3d());
    }
}
