//! Function associating an attribute value with a point in space.

use crate::prelude::{Attribute, Field};

/// Function associating an attribute value with a point in space.
///
/// API extension trait of `Field`;
/// moves the `Attr` generic into the function position,
/// and obscures the `attr` parameter using `Attribute`'s `Default` constraint
pub trait FieldAttribute {
    fn attribute<Attr>(&self, input: &Attr::Input) -> Attr::Output
    where
        Self: Field<Attr>,
        Attr: Attribute;
}

impl<T> FieldAttribute for T {
    fn attribute<Attr>(&self, input: &Attr::Input) -> Attr::Output
    where
        Self: Field<Attr>,
        Attr: Attribute,
    {
        self.field(input)
    }
}
