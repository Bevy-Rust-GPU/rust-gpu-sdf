pub mod raytrace;
pub mod sphere_trace_lipschitz;
pub mod sphere_trace_naive;

use rust_gpu_bridge::glam::Vec3;

use crate::default;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub struct RaycastOutput {
    /// True if the ray hit the target shape
    pub hit: bool,
    /// Minimum distance encountered between the ray and shape
    pub closest_dist: f32,
    /// Time at which the closest distance was encountered
    pub closest_t: f32,
    /// The amount of steps taken by this march
    pub steps: u32,
}

impl Default for RaycastOutput {
    fn default() -> Self {
        RaycastOutput {
            hit: default(),
            closest_dist: f32::MAX,
            closest_t: f32::MAX,
            steps: default(),
        }
    }
}

impl RaycastOutput {
    /// Notify the output that a step was taken at time `t`
    /// with a resulting distance of `dist`
    pub fn step(&mut self, t: f32, dist: f32) {
        if dist < self.closest_dist {
            self.closest_dist = dist;
            self.closest_t = t;
        }
    }

    /// Notify the output that marching ended in a hit at step `step`
    pub fn hit(&mut self, step: u32) {
        self.hit = true;
        self.steps = step;
    }

    /// Notify the output that marching ended in a miss at step `step`
    pub fn miss(&mut self, step: u32) {
        self.steps = step;
    }
}

/// Raycasting implementations for visualizing 3D signed distance fields.
pub trait Raycast<Sdf> {
    type Output;

    /// March from `eye` along the ray defined by `dir`,
    /// sampling `sdf` at discrete intervals and returning the result.
    fn raymarch(
        &self,
        sdf: &Sdf,
        start: f32,
        end: f32,
        eye: Vec3,
        dir: Vec3,
        epsilon: f32,
    ) -> Self::Output;
}
