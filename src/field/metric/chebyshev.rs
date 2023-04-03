//! Chebyshev distance metric.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs, Mix, Sign, Step,
};

use crate::prelude::{Distance, Field, Normal};

/// Chebyshev distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct ChebyshevMetric;

impl Field<Distance<f32>> for ChebyshevMetric {
    fn field(&self, p: &f32) -> f32 {
        p.abs()
    }
}

impl Field<Distance<Vec2>> for ChebyshevMetric {
    fn field(&self, p: &Vec2) -> f32 {
        p.x.abs().max(p.y.abs())
    }
}

impl Field<Distance<Vec3>> for ChebyshevMetric {
    fn field(&self, p: &Vec3) -> f32 {
        p.x.abs().max(p.y.abs()).max(p.z.abs())
    }
}

impl Field<Normal<f32>> for ChebyshevMetric {
    fn field(&self, p: &f32) -> f32 {
        p.sign()
    }
}

impl Field<Normal<Vec2>> for ChebyshevMetric {
    fn field(&self, p: &Vec2) -> Vec2 {
        let a = p.abs();
        let s = p.sign();

        (Vec2::X * s.x).mix(Vec2::Y * s.y, Vec2::splat(a.x.step(a.y)))
    }
}

impl Field<Normal<Vec3>> for ChebyshevMetric {
    fn field(&self, p: &Vec3) -> Vec3 {
        let a = p.abs();
        let s = p.sign();

        (Vec3::X * s.x)
            .mix(Vec3::Z * s.z, Vec3::splat(a.x.step(a.z)))
            .mix(
                (Vec3::Y * s.y).mix(Vec3::Z * s.z, Vec3::splat(a.y.step(a.z))),
                Vec3::splat(a.x.step(a.y)),
            )
            .into()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::glam::{Vec2, Vec3};

    use crate::prelude::{BoundTester, ChebyshevMetric};

    #[test]
    #[should_panic]
    pub fn test_chebyshev_metric_2d() {
        assert!(BoundTester::<ChebyshevMetric>::default().is_field_2d());
    }

    #[test]
    #[should_panic]
    pub fn test_chebyshev_metric_3d() {
        assert!(BoundTester::<ChebyshevMetric>::default().is_field_3d());
    }
}
