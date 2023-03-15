//! Chebyshev distance metric.

use rust_gpu_bridge::prelude::{Vec2, Vec3};

use crate::signed_distance_field::{Distance, SignedDistanceField};

/// Chebyshev distance metric.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChebyshevMetric;

impl SignedDistanceField<f32, Distance> for ChebyshevMetric {
    fn evaluate(&self, p: f32) -> Distance {
        p.abs().into()
    }
}

impl SignedDistanceField<Vec2, Distance> for ChebyshevMetric {
    fn evaluate(&self, p: Vec2) -> Distance {
        p.x.abs().max(p.y.abs()).into()
    }
}

impl SignedDistanceField<Vec3, Distance> for ChebyshevMetric {
    fn evaluate(&self, p: Vec3) -> Distance {
        p.x.abs().max(p.y.abs()).max(p.z.abs()).into()
    }
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::{Vec2, Vec3};

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
