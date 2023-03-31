//! Function associating an attribute value with a point in space.

pub mod metric;
pub mod shape;

pub mod field_operator;

pub mod field_attribute;

pub mod field_attributes;
pub mod fields;

use crate::prelude::Attribute;

/// Function associating an attribute value with a point in space.
pub trait Field<Pos, Attr>
where
    Attr: Attribute,
{
    fn field(&self, attr: Attr, p: Pos) -> Attr::Type;
}

#[cfg(feature = "glam")]
pub mod boxed {
    extern crate alloc;
    use alloc::boxed::Box;

    use crate::prelude::{Attribute, Field};

    impl<In, Attr> Field<In, Attr> for Box<dyn Field<In, Attr>>
    where
        Attr: Attribute,
    {
        fn field(&self, attr: Attr, p: In) -> <Attr as Attribute>::Type {
            self.as_ref().field(attr, p)
        }
    }
}
