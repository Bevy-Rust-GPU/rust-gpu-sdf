//! Function associating an attribute value with a point in space.

use crate::prelude::{Attribute, Field};

/// Function associating an attribute value with a point in space.
///
/// API extension trait of `Field`;
/// moves the `Attr` generic into the function position,
/// and obscures the `attr` parameter using `Attribute`'s `Default` constraint
pub trait FieldAttribute<In> {
    fn attribute<Attr>(&self, p: In) -> Attr::Type
    where
        Self: Field<In, Attr>,
        Attr: Default + Attribute;
}

impl<T, In> FieldAttribute<In> for T {
    fn attribute<Attr>(&self, p: In) -> Attr::Type
    where
        Self: Field<In, Attr>,
        Attr: Default + Attribute,
    {
        self.field(Attr::default(), p)
    }
}
