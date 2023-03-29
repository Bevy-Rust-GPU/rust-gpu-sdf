use rust_gpu_bridge::{glam::Vec3, Abs, Pow, Sign};
use type_fields::Field;

use crate::prelude::{Distance, FieldFunction, Normal};

// TODO: Apply pow(1.0 / foo) to un-exponentiate distance after axes are summed,
//       as per Superellipse
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct Superellipsoid {
    pub e1: f32,
    pub e2: f32,
}

impl Default for Superellipsoid {
    fn default() -> Self {
        Superellipsoid { e1: 1.0, e2: 1.0 }
    }
}

impl FieldFunction<Vec3, Distance> for Superellipsoid {
    fn evaluate(&self, _attr: Distance, p: Vec3) -> f32 {
        let d = (p.x.abs().pow(self.e1) + p.y.abs().pow(self.e2)).pow(self.e2 / self.e1)
            + p.z.abs().pow(self.e1);

        d - 1.0
    }
}

impl FieldFunction<Vec3, Normal<Vec3>> for Superellipsoid {
    fn evaluate(&self, _attr: Normal<Vec3>, p: Vec3) -> Vec3 {
        let pa = p.abs();
        let pp = pa.pow(Vec3::new(self.e1, self.e2, self.e1));
        pp.normalize() * p.sign()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::glam::Vec3;

    use crate::prelude::BoundChecker;

    use super::Superellipsoid;

    #[test]
    #[should_panic]
    fn test_superellipsoid() {
        assert!(BoundChecker::<Vec3, Superellipsoid>::default().is_field())
    }
}
