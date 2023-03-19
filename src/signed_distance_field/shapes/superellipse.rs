use rust_gpu_bridge::prelude::{Vec2, Abs, Pow};
use type_fields::Field;

use crate::signed_distance_field::{attributes::distance::Distance, DistanceFunction};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct Superellipse {
    pub n: f32,
}

impl Default for Superellipse {
    fn default() -> Self {
        Superellipse { n: 1.0 }
    }
}

impl DistanceFunction<Vec2, Distance> for Superellipse {
    fn evaluate(&self, p: Vec2) -> Distance {
        let d = p.x.abs().pow(self.n) + p.y.abs().pow(self.n);
        let d = d - 1.0;
        Distance(d)
    }
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::Vec2;

    use crate::prelude::BoundChecker;

    use super::Superellipse;

    #[test]
    #[should_panic]
    fn test_lame_curve() {
        assert!(BoundChecker::<Vec2, Superellipse>::default().is_field())
    }
}

