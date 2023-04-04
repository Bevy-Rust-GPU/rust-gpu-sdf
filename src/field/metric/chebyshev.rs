//! Chebyshev distance metric.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs, Mix, Sign, Step,
};

use crate::prelude::{AttrDistance, Field, AttrNormal, items::position::Position, Distance, Normal};

/// Chebyshev distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct ChebyshevMetric;

impl Field<AttrDistance<f32>> for ChebyshevMetric {
    fn field(&self, p: &Position<f32>) -> Distance {
        p.abs().into()
    }
}

impl Field<AttrDistance<Vec2>> for ChebyshevMetric {
    fn field(&self, p: &Position<Vec2>) -> Distance {
        p.x.abs().max(p.y.abs()).into()
    }
}

impl Field<AttrDistance<Vec3>> for ChebyshevMetric {
    fn field(&self, p: &Position<Vec3>) -> Distance {
        p.x.abs().max(p.y.abs()).max(p.z.abs()).into()
    }
}

impl Field<AttrNormal<f32>> for ChebyshevMetric {
    fn field(&self, p: &Position<f32>) -> Normal<f32> {
        p.sign().into()
    }
}

impl Field<AttrNormal<Vec2>> for ChebyshevMetric {
    fn field(&self, p: &Position<Vec2>) -> Normal<Vec2> {
        let a = p.abs();
        let s = p.sign();

        (Vec2::X * s.x).mix(Vec2::Y * s.y, Vec2::splat(a.x.step(a.y))).into()
    }
}

impl Field<AttrNormal<Vec3>> for ChebyshevMetric {
    fn field(&self, p: &Position<Vec3>) -> Normal<Vec3> {
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
