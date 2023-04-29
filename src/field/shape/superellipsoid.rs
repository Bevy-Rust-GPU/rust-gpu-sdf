use rust_gpu_bridge::{glam::Vec3, Abs, Pow, Sign};
use type_fields::macros::Field;

use crate::prelude::{items::position::Position, AttrDistance, Field, AttrNormal, Distance, Normal};

// TODO: Apply pow(1.0 / foo) to un-exponentiate distance after axes are summed,
//       as per Superellipse
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
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

impl Field<AttrDistance<Vec3>> for Superellipsoid {
    fn field(&self, p: &Position<Vec3>) -> Distance {
        let d = (p.x.abs().pow(self.e1) + p.y.abs().pow(self.e2)).pow(self.e2 / self.e1)
            + p.z.abs().pow(self.e1);

        (d - 1.0).into()
    }
}

impl Field<AttrNormal<Vec3>> for Superellipsoid {
    fn field(&self, p: &Position<Vec3>) -> Normal<Vec3> {
        let pa = p.abs();
        let pp = pa.pow(Vec3::new(self.e1, self.e2, self.e1));
        (pp.normalize() * p.sign()).into()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::prelude::BoundTester;

    use super::Superellipsoid;

    #[test]
    #[should_panic]
    fn test_superellipsoid() {
        assert!(BoundTester::<Superellipsoid>::default().is_field_3d())
    }
}
