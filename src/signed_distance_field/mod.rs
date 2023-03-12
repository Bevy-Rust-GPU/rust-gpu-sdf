//! Types that describe signed distance fields.

pub mod adapters;
pub mod metrics;
pub mod shapes;

use rust_gpu_bridge::prelude::Vec3;

/// Describes a shape in terms of distance to the nearest point on its surface.
pub trait SignedDistanceField<Dim> {
    fn distance(&self, p: Dim) -> f32;
}

/// Computes the normal at the nearest point on the surface of
/// its corresponding [`SignedDistanceField`].
pub trait SignedDistanceNormal<Dim>: SignedDistanceField<Dim> {
    fn normal(&self, p: Dim) -> Vec3;
}

impl<Dim> SignedDistanceField<Dim> for () {
    fn distance(&self, _: Dim) -> f32 {
        0.0
    }
}

impl<Dim> SignedDistanceField<Dim> for f32 {
    fn distance(&self, _: Dim) -> f32 {
        *self
    }
}

impl<F, Dim> SignedDistanceField<Dim> for F where F: Fn(Dim) -> f32 {
    fn distance(&self, p: Dim) -> f32 {
        self(p)
    }
}
