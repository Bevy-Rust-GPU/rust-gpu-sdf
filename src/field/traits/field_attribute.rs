use crate::prelude::{Attribute, Field};

/// Evalute the attribute `Attr` of a field function.
///
/// Moves `Attr` into the function position.
pub trait FieldAttribute {
    fn field_attribute<Attr>(&self, input: &Attr::Input) -> Attr::Output
    where
        Self: Field<Attr>,
        Attr: Attribute;
}

impl<T> FieldAttribute for T {
    fn field_attribute<Attr>(&self, input: &Attr::Input) -> Attr::Output
    where
        Self: Field<Attr>,
        Attr: Attribute,
    {
        self.field(input)
    }
}
