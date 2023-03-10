//! Types that describe signed distance fields.

pub mod metrics;
pub mod shapes;
pub mod adapters;

use rust_gpu_bridge::prelude::Vec3;

/// Describes a shape in terms of distance to the nearest point on its surface.
pub trait SignedDistanceField<Dim> {
    fn distance(&self, p: Dim) -> f32;
}

impl<T, Dim> SignedDistanceField<Dim> for &T
where
    T: SignedDistanceField<Dim>,
{
    fn distance(&self, p: Dim) -> f32 {
        <T as SignedDistanceField<Dim>>::distance(*self, p)
    }
}

/// Computes the normal at the nearest point on the surface of
/// its corresponding [`SignedDistanceField`].
pub trait SignedDistanceNormal<Dim>: SignedDistanceField<Dim> {
    fn normal(&self, p: Dim) -> Vec3;
}

impl<T, Dim> SignedDistanceNormal<Dim> for &T
where
    T: SignedDistanceNormal<Dim>,
{
    fn normal(&self, p: Dim) -> Vec3 {
        <T as SignedDistanceNormal<Dim>>::normal(*self, p)
    }
}
