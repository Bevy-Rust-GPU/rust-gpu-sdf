use rust_gpu_bridge::prelude::Vec2;

use crate::signed_distance_field::{attributes::distance::Distance, SignedDistanceField};

pub struct Ngon<const SIDES: usize, const FILL: bool> {
    pub cos: f32,
    pub sin: f32,
    pub tan: f32,
    pub radius: f32,
}

impl<const SIDES: usize, const FILL: bool> Default for Ngon<SIDES, FILL> {
    fn default() -> Self {
        Ngon {
            cos: (core::f32::consts::PI / SIDES as f32).cos(),
            sin: (core::f32::consts::PI / SIDES as f32).sin(),
            tan: (core::f32::consts::PI / SIDES as f32).tan(),
            radius: 1.0,
        }
    }
}

impl<const SIDES: usize, const FILL: bool> SignedDistanceField<Vec2, Distance>
    for Ngon<SIDES, FILL>
{
    fn evaluate(&self, p: Vec2) -> Distance {
        let mut p = p;

        // Mirror to reduce total vertex evaluations
        if SIDES % 2 == 0 {
            // Shapes with an even number of sides can be mirrored in XY
            p = p.abs();
        } else {
            // Shapes with an odd number of sides can be mirrored in X
            p.x = p.x.abs();
        }

        // Iteratively add vertices by mirroring distance space:
        // Every 2 sides, mirror along the exterior angle of our n-gon,
        // flipping the horizontal mirroring plane every other step.
        for (i, _) in (0..SIDES - 2).step_by(2).enumerate() {
            let sign = (i % 2) as f32 * 2.0 - 1.0;
            p -= 2.0
                * Vec2::new(sign * self.cos, self.sin).dot(p).min(0.0) // Plane interior
                * Vec2::new(sign * self.cos, self.sin);
        }

        // Apply 1D line to X component, isosurface offset to Y
        p -= Vec2::new(
            p.x.clamp(-self.tan * self.radius, self.tan * self.radius),
            self.radius,
        );

        // Evaluate euclidean metric
        let mut d = p.length();

        // Apply sidedness
        if FILL {
            d *= p.y.signum();
        }

        // Done
        d.into()
    }
}

