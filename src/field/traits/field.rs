use crate::prelude::Attribute;

/// Function associating an attribute value with a point in space.
pub trait Field<Attr>
where
    Attr: Attribute,
{
    fn field(&self, input: &Attr::Input) -> Attr::Output;
}

