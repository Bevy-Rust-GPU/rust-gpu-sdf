use rust_gpu_bridge::{glam::Vec3, Abs};

use crate::prelude::{Distance, FieldFunction};

use super::{Raycast, RaycastOutput};

/// Basic sphere tracer.
///
/// Marches along a ray, sampling the provided SDF at each step to determine
/// a minimum safe distance for the following iteration.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct SphereTraceNaive<const MAX_STEPS: u32>;

impl<const MAX_STEPS: u32, Sdf> Raycast<Sdf> for SphereTraceNaive<MAX_STEPS>
where
    Sdf: FieldFunction<Vec3, Distance>,
{
    type Output = RaycastOutput;

    fn raymarch(
        &self,
        sdf: &Sdf,
        start: f32,
        end: f32,
        eye: Vec3,
        dir: Vec3,
        epsilon: f32,
    ) -> Self::Output {
        let mut out = RaycastOutput::default();
        let mut t = start;

        for step in 0..MAX_STEPS {
            let p = eye + dir * t;
            let dist = sdf.evaluate(Distance, p);

            out.step(t, dist);

            if dist < 0.0 {
                out.hit(step);
                break;
            }

            t += epsilon.max(dist.abs());

            if t > end {
                out.miss(step);
                break;
            }
        }

        out
    }
}
