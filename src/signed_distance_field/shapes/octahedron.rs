//! An octahedron.

use rust_gpu_bridge::prelude::Vec3;
use type_fields::Field;

use crate::signed_distance_field::{Distance, SignedDistanceField};

/// An octahedron.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct Octahedron {
    pub size: f32,
}

impl Default for Octahedron {
    fn default() -> Self {
        Octahedron { size: 1.0 }
    }
}

impl SignedDistanceField<Vec3, Distance> for Octahedron {
    fn evaluate(&self, p: Vec3) -> Distance {
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

#[cfg(test)]
pub mod test {
    use type_fields::field::Field;

    use super::Octahedron;

    #[test]
    fn test_octahedron() {
        Octahedron::default().with(Octahedron::size, f32::default());
    }
}
