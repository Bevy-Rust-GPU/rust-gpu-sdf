use rust_gpu_bridge::prelude::{Vec3, Abs, Pow};
use type_fields::Field;

use crate::signed_distance_field::{attributes::distance::Distance, SignedDistanceField};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct Superellipsoid {
    pub e1: f32,
    pub e2: f32,
}

impl Default for Superellipsoid {
    fn default() -> Self {
        Superellipsoid { e1: 1.0, e2: 1.0 }
    }
}

impl SignedDistanceField<Vec3, Distance> for Superellipsoid {
    fn evaluate(&self, p: Vec3) -> Distance {
        let d = (p.x.abs().pow(self.e1) + p.y.abs().pow(self.e2)).pow(self.e2 / self.e1)
            + p.z.abs().pow(self.e1);
        let d = d - 1.0;
        Distance(d)
    }
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::Vec3;

    use crate::prelude::BoundChecker;

    use super::Superellipsoid;

    #[test]
    #[should_panic]
    fn test_superellipsoid() {
        assert!(BoundChecker::<Vec3, Superellipsoid>::default().is_field())
    }
}
