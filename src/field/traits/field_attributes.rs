//! Function associating several attribute values with a point in space.

use type_fields::cons::Uncons;

use crate::prelude::{Attributes, ConsAttributes, Fields};

/// Function associating several attribute values with a point in space.
///
/// Extension trait of `Fields`;
/// `Cons`es the provided tuple, evaluates it via `FieldAttributesImpl`,
/// then returns the `Uncons` of the result.
pub trait FieldAttributes {
    fn attributes<'a, Attr>(
        &self,
        input: &<Attr::ConsAttr as Attributes>::Input,
    ) -> Attr::UnconsOutput
    where
        Self: Fields<Attr::ConsAttr>,
        Attr: ConsAttributes<'a>;
}

impl<T> FieldAttributes for T {
    fn attributes<'a, Attr>(&self, input: &<Attr::Cons as Attributes>::Input) -> Attr::UnconsOutput
    where
        Self: Fields<Attr::Cons>,
        Attr: ConsAttributes<'a>,
    {
        self.fields(input).uncons()
    }
}
