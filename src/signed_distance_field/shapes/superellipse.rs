use rust_gpu_bridge::{glam::Vec2, Abs, Pow};
use type_fields::Field;

use crate::prelude::{Distance, FieldFunction};

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

impl FieldFunction<Vec2, Distance> for Superellipse {
    fn evaluate(&self, attr: Distance, p: Vec2) -> f32 {
        p.x.abs().pow(self.n) + p.y.abs().pow(self.n) - 1.0
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::glam::Vec2;

    use crate::prelude::BoundChecker;

    use super::Superellipse;

    #[test]
    #[should_panic]
    fn test_lame_curve() {
        assert!(BoundChecker::<Vec2, Superellipse>::default().is_field())
    }
}
