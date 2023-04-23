use crate::prelude::Attribute;

/// Evalute the attribute `Attr` of a field function.
pub trait Field<Attr>
where
    Attr: Attribute,
{
    fn field(&self, input: &Attr::Input) -> Attr::Output;
}

