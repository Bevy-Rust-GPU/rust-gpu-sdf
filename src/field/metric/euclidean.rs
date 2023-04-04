//! Euclidean distance metric.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Asin, Atan2, Length, Normalize,
};

use crate::prelude::{
    items::position::Position, AttrDistance, AttrNormal, AttrUv, Distance, Field, Normal, Uv,
};

/// Euclidian distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[cfg_attr(feature = "bevy", derive(bevy::reflect::TypeUuid))]
#[cfg_attr(feature = "bevy", uuid = "9c0e79ee-c437-47aa-9230-d3c13f3bacb8")]
#[repr(C)]
pub struct EuclideanMetric;

impl<Input> Field<AttrDistance<Input>> for EuclideanMetric
where
    Input: Clone + Length,
{
    fn field(&self, input: &Position<Input>) -> Distance {
        (**input).clone().length().into()
    }
}

impl<Dim> Field<AttrNormal<Dim>> for EuclideanMetric
where
    Dim: Default + Clone + PartialEq + Normalize,
{
    fn field(&self, input: &Position<Dim>) -> Normal<Dim> {
        let d = Dim::default();
        if **input == d {
            return d.into();
        }

        (**input).clone().normalize().into()
    }
}

impl Field<AttrUv<f32>> for EuclideanMetric {
    fn field(&self, p: &Position<f32>) -> Uv {
        Vec2::new(**p, 0.0).into()
    }
}

impl Field<AttrUv<Vec2>> for EuclideanMetric {
    fn field(&self, p: &Position<Vec2>) -> Uv {
        Vec2::new((p.x.atan2(p.y) / core::f32::consts::TAU) + 0.5, p.length()).into()
    }
}

impl Field<AttrUv<Vec3>> for EuclideanMetric {
    fn field(&self, p: &Position<Vec3>) -> Uv {
        Vec2::new(
            (p.x.atan2(p.z) / core::f32::consts::TAU) + 0.5,
            (p.y.asin() / core::f32::consts::PI) + 0.5,
        )
        .into()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::prelude::{BoundTester, EuclideanMetric};

    #[test]
    fn test_euclidean_metric_2d() {
        assert!(BoundTester::<EuclideanMetric>::default().is_field_2d());
    }

    #[test]
    fn test_euclidean_metric_3d() {
        assert!(BoundTester::<EuclideanMetric>::default().is_field_3d());
    }
}
