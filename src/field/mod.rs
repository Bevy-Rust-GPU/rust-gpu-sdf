//! Function associating an attribute value with a point in space.

pub mod metric;
pub mod shape;

pub mod field_operator;

pub mod field_attribute;

pub mod field_attributes;
pub mod fields;

use crate::prelude::Attribute;

/// Function associating an attribute value with a point in space.
pub trait Field<Attr>
where
    Attr: Attribute,
{
    fn field(&self, input: &Attr::Input) -> Attr::Output;
}

#[cfg(feature = "glam")]
pub mod boxed {
    extern crate alloc;
    use alloc::boxed::Box;

    use crate::prelude::{Attribute, Field};

    impl<Attr> Field<Attr> for Box<dyn Field<Attr>>
    where
        Attr: Attribute,
    {
        fn field(&self, input: &<Attr as Attribute>::Input) -> <Attr as Attribute>::Output {
            self.as_ref().field(input)
        }
    }
}
