//! Types that describe signed distance fields.

pub mod adapters;
pub mod attributes;
pub mod metrics;
pub mod shapes;

use self::attributes::{distance::Distance};

/// Describes a shape in terms of distance to the nearest point on its surface.
pub trait DistanceFunction<In, Out> {
    fn evaluate(&self, p: In) -> Out;
}

impl<Dim> DistanceFunction<Dim, Distance> for () {
    fn evaluate(&self, _: Dim) -> Distance {
        0.0.into()
    }
}

impl<Dim> DistanceFunction<Dim, Distance> for f32 {
    fn evaluate(&self, _: Dim) -> Distance {
        (*self).into()
    }
}

impl<F, In, Out> DistanceFunction<In, Out> for F
where
    F: Fn(In) -> Out,
{
    fn evaluate(&self, p: In) -> Out {
        self(p)
    }
}
