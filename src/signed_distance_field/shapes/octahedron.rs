//! An octahedron.

use rust_gpu_bridge::prelude::Vec3;

use crate::signed_distance_field::SignedDistanceField;

/// An octahedron.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Octahedron {
    size: f32,
}

impl Default for Octahedron {
    fn default() -> Self {
        Octahedron { size: 1.0 }
    }
}

impl SignedDistanceField<Vec3> for Octahedron {
    fn distance(&self, p: Vec3) -> f32 {
        let p = p.abs();
        let m = p.x + p.y + p.z - self.size;
        let q = if 3.0 * p.x < m {
            p
        } else if 3.0 * p.y < m {
            Vec3::new(p.y, p.z, p.x)
        } else if 3.0 * p.z < m {
            Vec3::new(p.z, p.x, p.y)
        } else {
            return m * 0.57735027;
        };

        let k = (0.5 * (q.z - q.y + self.size)).clamp(0.0, self.size);
        Vec3::new(q.x, q.y - self.size + k, q.z - k).length()
    }
}

