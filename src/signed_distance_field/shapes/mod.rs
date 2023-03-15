//! Distance field shapes.

pub mod composite;
pub mod octahedron;
pub mod plane;
pub mod squircle;
pub mod lame_curve {
    use rust_gpu_bridge::prelude::Vec2;
    use type_fields::Field;

    use crate::signed_distance_field::{attributes::distance::Distance, SignedDistanceField};

    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
    pub struct LameCurve {
        pub n: f32,
    }

    impl Default for LameCurve {
        fn default() -> Self {
            LameCurve { n: 1.0 }
        }
    }

    impl SignedDistanceField<Vec2, Distance> for LameCurve {
        fn evaluate(&self, p: Vec2) -> Distance {
            let d = p.x.abs().powf(self.n) + p.y.abs().powf(self.n);
            let d = d - 1.0;
            Distance(d)
        }
    }

    #[cfg(test)]
    pub mod test {
        use rust_gpu_bridge::prelude::Vec2;

        use crate::prelude::BoundChecker;

        use super::LameCurve;

        #[test]
        #[should_panic]
        fn test_lame_curve() {
            assert!(BoundChecker::<Vec2, LameCurve>::default().is_field())
        }
    }
}
