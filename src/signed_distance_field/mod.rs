//! Types that describe signed distance fields.

pub mod adapters;
pub mod metrics;
pub mod shapes;

/*
use core::ops::{Deref, DerefMut};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Distance(pub f32);

impl Deref for Distance {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Distance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
*/

/// Describes a shape in terms of distance to the nearest point on its surface.
pub trait SignedDistanceField<In, Out> {
    fn evaluate(&self, p: In) -> Out;
}

impl<Dim> SignedDistanceField<Dim, f32> for () {
    fn evaluate(&self, _: Dim) -> f32 {
        0.0
    }
}

impl<Dim> SignedDistanceField<Dim, f32> for f32 {
    fn evaluate(&self, _: Dim) -> f32 {
        *self
    }
}

impl<F, In, Out> SignedDistanceField<In, Out> for F
where
    F: Fn(In) -> Out,
{
    fn evaluate(&self, p: In) -> Out {
        self(p)
    }
}
