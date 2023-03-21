use rust_gpu_bridge::{glam::Vec3, Abs};

use crate::prelude::{Distance, DistanceFunction};

use super::{Raymarch, RaymarchOutput};

/// Sphere tracer that operates with respect to a precomputed Lipschitz bound.
///
/// Costs 1 extra divide per step compared to [`SphereTraceNaive`],
/// but results in overall faster intersections.
///
/// Note: The precomputed lipschitz bound `k` must be correct in respect to the
/// provided SDF for accurate behaviour; incorrect values will result in visual artifacting.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct SphereTraceLipschitz<const MAX_STEPS: u32> {
    pub frac_1_k: f32,
}

impl<const MAX_STEPS: u32> Default for SphereTraceLipschitz<MAX_STEPS> {
    fn default() -> Self {
        SphereTraceLipschitz {
            frac_1_k: 1.0 / (SphereTraceLipschitz::<MAX_STEPS>::falloff_k(1.0, 3.0) * 3.0),
        }
    }
}

impl<const MAX_STEPS: u32> SphereTraceLipschitz<MAX_STEPS> {
    // Computes the global lipschitz bound of the falloff function
    // e: energy
    // R: radius
    fn falloff_k(e: f32, r: f32) -> f32 {
        1.72 * e.abs() / r
    }
}

impl<const MAX_STEPS: u32> Raymarch for SphereTraceLipschitz<MAX_STEPS> {
    type Output = RaymarchOutput;

    fn raymarch<Sdf>(
        &self,
        sdf: &Sdf,
        start: f32,
        end: f32,
        eye: Vec3,
        dir: Vec3,
        epsilon: f32,
    ) -> Self::Output
    where
        Sdf: DistanceFunction<Vec3, Distance>,
    {
        let mut out = RaymarchOutput::default();

        let mut t = start;
        for i in 0..MAX_STEPS {
            let p = eye + dir * t;
            let dist: Distance = sdf.evaluate(p);

            out.step(t, dist);

            if *dist < 0.0 {
                out.hit(i);
                break;
            }

            t += epsilon.max(dist.abs() * self.frac_1_k);

            if t > end {
                out.miss(i);
                break;
            }
        }

        out
    }
}
