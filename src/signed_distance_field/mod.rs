//! Types that describe signed distance fields.

pub mod adapters;
pub mod attributes;
pub mod metrics;
pub mod shapes;

use self::attributes::{distance::Distance, Attribute};

/// Describes a shape in terms of distance to the nearest point on its surface.
pub trait DistanceFunction<In, Attr>
where
    Attr: Attribute,
{
    fn evaluate(&self, attr: Attr, p: In) -> Attr::Type;
}

impl<Dim> DistanceFunction<Dim, Distance> for () {
    fn evaluate(&self, _: Distance, _: Dim) -> f32 {
        0.0
    }
}

impl<Dim> DistanceFunction<Dim, Distance> for f32 {
    fn evaluate(&self, _: Distance, _: Dim) -> f32 {
        *self
    }
}

impl<F, In, Attr> DistanceFunction<In, Attr> for F
where
    Attr: Attribute,
    F: Fn(In) -> Attr::Type,
{
    fn evaluate(&self, _: Attr, p: In) -> Attr::Type {
        self(p)
    }
}
