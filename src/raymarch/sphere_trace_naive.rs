use rust_gpu_bridge::prelude::{Vec3, Abs};

use crate::prelude::{Distance, SignedDistanceField};

use super::{Raymarch, RaymarchOutput};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SphereTraceNaive;

impl Raymarch for SphereTraceNaive {
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
            let dist = *sdf.evaluate(p);

            out.steps += 1;

            if dist < 0.0 {
                out.hit = true;
                out.dist = t;
                break;
            }

            t += epsilon.max(dist.abs());

            if t > end {
                out.dist = end;
                break;
            }
        }

        out
    }
}

