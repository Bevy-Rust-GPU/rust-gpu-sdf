//! Distance field shapes.

pub mod composite;
pub mod octahedron;
pub mod plane;
pub mod squircle;
pub mod superellipse;
pub mod superellipsoid {
    use rust_gpu_bridge::prelude::Vec3;
    use type_fields::Field;

    use crate::signed_distance_field::{attributes::distance::Distance, SignedDistanceField};

    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
    pub struct Superellipsoid {
        pub e1: f32,
        pub e2: f32,
    }

    impl SignedDistanceField<Vec3, Distance> for Superellipsoid {
        fn evaluate(&self, p: Vec3) -> Distance {
            let d = (p.x.abs().powf(self.e1) + p.y.abs().powf(self.e2)).powf(self.e2 / self.e1)
                + p.z.abs().powf(self.e1);
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
}
