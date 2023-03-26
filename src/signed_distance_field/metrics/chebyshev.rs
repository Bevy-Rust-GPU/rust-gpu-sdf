//! Chebyshev distance metric.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs, Mix, Sign, Step,
};

use crate::signed_distance_field::{attributes::normal::Normal, Distance, DistanceFunction};

/// Chebyshev distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ChebyshevMetric;

impl DistanceFunction<f32, Distance> for ChebyshevMetric {
    fn evaluate(&self, attr: Distance, p: f32) -> f32 {
        p.abs()
    }
}

impl DistanceFunction<Vec2, Distance> for ChebyshevMetric {
    fn evaluate(&self, attr: Distance, p: Vec2) -> f32 {
        p.x.abs().max(p.y.abs())
    }
}

impl DistanceFunction<Vec3, Distance> for ChebyshevMetric {
    fn evaluate(&self, attr: Distance, p: Vec3) -> f32 {
        p.x.abs().max(p.y.abs()).max(p.z.abs())
    }
}

impl DistanceFunction<f32, Normal<f32>> for ChebyshevMetric {
    fn evaluate(&self, attr: Normal<f32>, p: f32) -> f32 {
        p.sign()
    }
}

impl DistanceFunction<Vec2, Normal<Vec2>> for ChebyshevMetric {
    fn evaluate(&self, attr: Normal<Vec2>, p: Vec2) -> Vec2 {
        let a = p.abs();
        let s = p.sign();

        (Vec2::X * s.x).mix(Vec2::Y * s.y, Vec2::splat(a.x.step(a.y)))
    }
}

impl DistanceFunction<Vec3, Normal<Vec3>> for ChebyshevMetric {
    fn evaluate(&self, attr: Normal<Vec3>, p: Vec3) -> Vec3 {
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

    use crate::{
        prelude::BoundChecker, signed_distance_field::metrics::chebyshev::ChebyshevMetric,
    };

    #[test]
    #[should_panic]
    pub fn test_chebyshev_metric_2d() {
        assert!(BoundChecker::<Vec2, ChebyshevMetric>::default().is_field());
    }

    #[test]
    #[should_panic]
    pub fn test_chebyshev_metric_3d() {
        assert!(BoundChecker::<Vec3, ChebyshevMetric>::default().is_field());
    }
}
