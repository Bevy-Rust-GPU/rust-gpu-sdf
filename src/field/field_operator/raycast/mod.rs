//! Operators dedicating to visualizing 3D distance functions via ray intersection

pub mod raytrace;
pub mod sphere_trace_lipschitz;
pub mod sphere_trace_naive;

use rust_gpu_bridge::glam::Vec3;

use crate::{default, prelude::Attribute};

#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct RaycastInput {
    pub eye: Vec3,
    pub dir: Vec3,
    pub start: f32,
    pub end: f32,
}

impl Default for RaycastInput {
    fn default() -> Self {
        RaycastInput {
            eye: Vec3::ZERO,
            dir: -Vec3::Z,
            start: 0.0,
            end: 1000.0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub struct RaycastOutput {
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
            closest_dist: f32::MAX,
            closest_t: f32::MAX,
            steps: default(),
        }
    }
}

impl RaycastOutput {
    /// Notify the output that a step was taken at time `t`
    /// with a resulting distance of `dist`
    pub fn march_step(&mut self, t: f32, dist: f32) {
        if dist < self.closest_dist {
            self.closest_dist = dist;
            self.closest_t = t;
        }
    }

    /// Notify the output that marching ended in a hit at step `step`
    pub fn march_hit(&mut self, step: u32) {
        self.steps = step;
    }

    /// Notify the output that marching ended in a miss at step `step`
    pub fn march_miss(&mut self, step: u32) {
        self.steps = step;
    }

    pub fn hit(&self) -> bool {
        self.closest_dist <= 0.0
    }
}

impl Attribute for RaycastOutput {
    type Type = RaycastOutput;
}
