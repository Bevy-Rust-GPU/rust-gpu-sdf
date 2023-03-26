//! Analytical raytracer.

use rust_gpu_bridge::glam::Vec3;

use super::{Raycast, RaycastOutput};

/// Compute the intersection between self and the given ray
pub trait RayIntersection {
    fn intersect(&self, eye: Vec3, dir: Vec3) -> Option<f32>;
}

/// Analytical raytracer.
///
/// Evaluates the [`RayIntersection`] of the provided type.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Raytrace;

impl<Sdf> Raycast<Sdf> for Raytrace
where
    Sdf: RayIntersection,
{
    type Output = RaycastOutput;

    fn raymarch(
        &self,
        sdf: &Sdf,
        start: f32,
        _: f32,
        eye: Vec3,
        dir: Vec3,
        _: f32,
    ) -> Self::Output {
        let mut out = RaycastOutput::default();
        out.steps = 1;

        if let Some(t) = sdf.intersect(eye + dir * start, dir) {
            out.hit = true;
            out.closest_t = t;
            out.closest_dist = 0.0;
        }

        out
    }
}

