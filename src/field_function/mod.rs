pub mod metric;
pub mod shape;

use crate::{default, prelude::Attribute};

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

#[cfg(feature = "glam")]
pub mod boxed {
    extern crate alloc;
    use alloc::boxed::Box;

    use crate::prelude::{Attribute, FieldFunction};

    impl<In, Attr> FieldFunction<In, Attr> for Box<dyn FieldFunction<In, Attr>>
    where
        Attr: Attribute,
    {
        fn evaluate(&self, attr: Attr, p: In) -> <Attr as Attribute>::Type {
            self.as_ref().evaluate(attr, p)
        }
    }
}
