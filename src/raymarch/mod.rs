pub mod sphere_trace_lipschitz;
pub mod sphere_trace_naive;

use rust_gpu_bridge::prelude::Vec3;

use crate::prelude::{Distance, SignedDistanceField};

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub struct RaymarchOutput {
    pub hit: bool,
    pub dist: f32,
    pub steps: u32,
}

pub trait Raymarch {
    type Output;

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
        Sdf: SignedDistanceField<Vec3, Distance>;
}
