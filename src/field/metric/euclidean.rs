//! Euclidean distance metric.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Asin, Atan2, Length, Normalize,
};

use crate::prelude::{Distance, Field, Normal, Uv};

/// Euclidian distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[cfg_attr(feature = "bevy", derive(bevy::reflect::TypeUuid))]
#[cfg_attr(feature = "bevy", uuid = "9c0e79ee-c437-47aa-9230-d3c13f3bacb8")]
#[repr(C)]
pub struct EuclideanMetric;

impl<Input> Field<Distance<Input>> for EuclideanMetric
where
    Input: Clone + Length,
{
    fn field(&self, input: &Input) -> f32 {
        input.clone().length()
    }
}

impl<Input> Field<Normal<Input>> for EuclideanMetric
where
    Input: Default + Clone + PartialEq + Normalize,
{
    fn field(&self, input: &Input) -> Input {
        let d = Input::default();
        if *input == d {
            return d;
        }

        input.clone().normalize()
    }
}

impl Field<Uv<f32>> for EuclideanMetric {
    fn field(&self, p: &f32) -> Vec2 {
        Vec2::new(*p, 0.0)
    }
}

impl Field<Uv<Vec2>> for EuclideanMetric {
    fn field(&self, p: &Vec2) -> Vec2 {
        Vec2::new((p.x.atan2(p.y) / core::f32::consts::TAU) + 0.5, p.length())
    }
}

impl Field<Uv<Vec3>> for EuclideanMetric {
    fn field(&self, p: &Vec3) -> Vec2 {
        Vec2::new(
            (p.x.atan2(p.z) / core::f32::consts::TAU) + 0.5,
            (p.y.asin() / core::f32::consts::PI) + 0.5,
        )
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
