//! Types that describe signed distance fields.

pub mod metrics;
pub mod shapes;

use rust_gpu_bridge::prelude::Vec3;

/// Describes a shape in terms of distance to the nearest point on its surface.
pub trait SignedDistanceField<P> {
    fn distance(&self, p: P) -> f32;
}

impl<T, P> SignedDistanceField<P> for &T
where
    T: SignedDistanceField<P>,
{
    fn distance(&self, p: P) -> f32 {
        <T as SignedDistanceField<P>>::distance(*self, p)
    }
}

/// Computes the normal at the nearest point on the surface of
/// its corresponding [`SignedDistanceField`].
pub trait SignedDistanceNormal<P>: SignedDistanceField<P> {
    fn normal(&self, p: P) -> Vec3;
}

impl<T, P> SignedDistanceNormal<P> for &T
where
    T: SignedDistanceNormal<P>,
{
    fn normal(&self, p: P) -> Vec3 {
        <T as SignedDistanceNormal<P>>::normal(*self, p)
    }
}
