use type_fields::t_funk::hlist::ToTList;

use crate::prelude::{Attributes, ConsAttributes, Fields};

/// Evalute multiple attributes of a field function.
///
/// Moves `Attrs` into the function position.
pub trait FieldAttributes {
    fn field_attributes<'a, Attrs>(
        &self,
        input: &<Attrs::ConsAttr as Attributes>::Input,
    ) -> Attrs::UnconsOutput
    where
        Self: Fields<Attrs::ConsAttr>,
        Attrs: ConsAttributes<'a>;
}

impl<T> FieldAttributes for T {
    fn field_attributes<'a, Attr>(
        &self,
        input: &<Attr::HList as Attributes>::Input,
    ) -> Attr::UnconsOutput
    where
        Self: Fields<Attr::HList>,
        Attr: ConsAttributes<'a>,
    {
        self.fields(input).to_tlist()
    }
}
