//! Function associating several attribute values with a point in space.

use type_fields::cons::{Cons, Uncons};

use crate::prelude::{Attributes, Fields};

/// Function associating several attribute values with a point in space.
///
/// Extension trait of `Fields`;
/// `Cons`es the provided tuple, evaluates it via `FieldAttributesImpl`,
/// then returns the `Uncons` of the result.
pub trait FieldAttributes<In> {
    fn attributes<Attr>(&self, p: In) -> <<Attr::Cons as Attributes>::Type as Uncons>::Uncons
    where
        Self: Fields<In, Attr::Cons>,
        Attr: Default + Cons,
        Attr::Cons: Attributes,
        <Attr::Cons as Attributes>::Type: Uncons;
}

impl<T, In> FieldAttributes<In> for T {
    fn attributes<Attr>(&self, p: In) -> <<Attr::Cons as Attributes>::Type as Uncons>::Uncons
    where
        Self: Fields<In, Attr::Cons>,
        Attr: Default + Cons,
        Attr::Cons: Attributes,
        <Attr::Cons as Attributes>::Type: Uncons,
    {
        self.fields(Attr::default().cons(), p).uncons()
    }
}
