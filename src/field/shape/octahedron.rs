//! An octahedron.

use rust_gpu_bridge::{glam::Vec3, Normalize, Sign};
use type_fields::macros::Field;

use crate::prelude::{
    items::position::Position, AttrDistance, AttrNormal, Distance, Field, Normal,
};

/// An octahedron.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct Octahedron {
    pub size: f32,
}

impl Default for Octahedron {
    fn default() -> Self {
        Octahedron { size: 1.0 }
    }
}

impl Field<AttrDistance<Vec3>> for Octahedron {
    fn field(&self, p: &Position<Vec3>) -> Distance {
        // Axial reflection
        let p = p.abs();

        // Signed distance minus size
        let m = p.x + p.y + p.z - self.size;

        let q = if 3.0 * p.x < m {
            p
        } else if 3.0 * p.y < m {
            Vec3::new(p.y, p.z, p.x)
        } else if 3.0 * p.z < m {
            Vec3::new(p.z, p.x, p.y)
        } else {
            return (m * 0.57735027).into();
        };

        let k = (0.5 * (q.z - q.y + self.size)).clamp(0.0, self.size);

        let j = Vec3::new(q.x, q.y - self.size + k, q.z - k);

        // Euclidean metric
        j.length().into()
    }
}

impl<Dim> Field<AttrNormal<Dim>> for Octahedron
where
    Dim: Clone + Sign + Normalize,
{
    fn field(&self, p: &Position<Dim>) -> Normal<Dim> {
        (**p).clone().sign().normalize().into()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::prelude::BoundTester;

    use super::Octahedron;

    #[test]
    fn test_octahedron() {
        assert!(BoundTester::<Octahedron>::default().is_field_3d())
    }
}
