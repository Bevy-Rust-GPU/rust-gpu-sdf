use crate::prelude::{Attribute, Distance};

/// Describes a shape in terms of distance to the nearest point on its surface.
pub trait FieldFunction<In, Attr>
where
    Attr: Attribute,
{
    fn evaluate(&self, attr: Attr, p: In) -> Attr::Type;
}

impl<Dim> FieldFunction<Dim, Distance> for () {
    fn evaluate(&self, _: Distance, _: Dim) -> f32 {
        0.0
    }
}

impl<Dim> FieldFunction<Dim, Distance> for f32 {
    fn evaluate(&self, _: Distance, _: Dim) -> f32 {
        *self
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

