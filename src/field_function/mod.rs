pub mod metric;
pub mod shape;

use rust_gpu_bridge::glam::{Vec3, Vec2};

use crate::{
    default,
    prelude::{Attribute, Normal},
};

/// Describes a shape in terms of distance to the nearest point on its surface.
pub trait FieldFunction<In, Attr>
where
    Attr: Attribute,
{
    fn evaluate(&self, attr: Attr, p: In) -> Attr::Type;
}

impl<Dim, Attr> FieldFunction<Dim, Attr> for ()
where
    Attr: Attribute,
    Attr::Type: Default,
{
    fn evaluate(&self, _: Attr, _: Dim) -> Attr::Type {
        default()
    }
}

impl<F, In, Attr> FieldFunction<In, Attr> for F
where
    Attr: Attribute,
    F: Fn(In) -> Attr::Type,
{
    fn evaluate(&self, _: Attr, p: In) -> Attr::Type {
        self(p)
    }
}
