use rust_gpu_bridge::{glam::Vec2, Abs, Pow, Sign};
use type_fields::Field;

use crate::prelude::{Distance, Field, Normal};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct Superellipse {
    pub n: f32,
}

impl Default for Superellipse {
    fn default() -> Self {
        Superellipse { n: 1.0 }
    }
}

impl Field<Distance<Vec2>> for Superellipse {
    fn field(&self, p: Vec2) -> f32 {
        (p.x.abs().pow(self.n) + p.y.abs().pow(self.n)).pow(1.0 / self.n)
    }
}

impl Field<Normal<Vec2>> for Superellipse {
    fn field(&self, p: Vec2) -> Vec2 {
        p.abs().pow(Vec2::splat(self.n)).normalize() * p.sign()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::prelude::BoundTester;

    use super::Superellipse;

    #[test]
    #[should_panic]
    fn test_lame_curve() {
        assert!(BoundTester::<Superellipse>::default().is_field_2d())
    }
}
