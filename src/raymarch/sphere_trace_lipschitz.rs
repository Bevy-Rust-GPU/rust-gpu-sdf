use rust_gpu_bridge::prelude::{Vec3, Abs};

use crate::prelude::{Distance, SignedDistanceField};

use super::{Raymarch, RaymarchOutput};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SphereTraceLipschitz {
    pub k: f32,
}

impl Default for SphereTraceLipschitz {
    fn default() -> Self {
        SphereTraceLipschitz {
            k: SphereTraceLipschitz::falloff_k(1.0, 3.0) * 3.0,
        }
    }
}

impl SphereTraceLipschitz {
    // Computes the global lipschitz bound of the falloff function
    // e: energy
    // R: radius
    fn falloff_k(e: f32, r: f32) -> f32 {
        1.72 * e.abs() / r
    }
}

impl Raymarch for SphereTraceLipschitz {
    type Output = RaymarchOutput;

    fn raymarch<Sdf, const MAX_STEPS: u32>(
        &self,
        sdf: &Sdf,
        start: f32,
        end: f32,
        eye: Vec3,
        dir: Vec3,
        epsilon: f32,
    ) -> Self::Output
    where
        Sdf: SignedDistanceField<Vec3, Distance>,
    {
        let mut out = RaymarchOutput::default();

        let mut t = start;

        for _ in 0..MAX_STEPS {
            let p = eye + dir * t;
            let dist = sdf.evaluate(p);

            out.steps += 1;

            if *dist < 0.0 {
                out.hit = true;
                out.dist = t;
                break;
            }

            t += epsilon.max(dist.abs() / self.k);

            if t > end {
                out.dist = end;
                break;
            }
        }

        out
    }
}

