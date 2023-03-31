//! Function associating several attribute values with a point in space.

use type_fields::cons::{Cons, Uncons};

use crate::prelude::{Attributes, Fields};

/// Function associating several attribute values with a point in space.
///
/// Extension trait of `Fields`;
/// `Cons`es the provided tuple, evaluates it via `FieldAttributesImpl`,
/// then returns the `Uncons` of the result.
pub trait FieldAttributes<In, const COUNT: usize> {
    fn attributes<Attr>(
        &self,
        p: In,
    ) -> <<Attr::Cons as Attributes>::Type as Uncons<COUNT>>::Uncons
    where
        Self: Fields<In, Attr::Cons>,
        Attr: Default + Cons,
        Attr::Cons: Attributes,
        <Attr::Cons as Attributes>::Type: Uncons<COUNT>;
}

impl<T, In, const COUNT: usize> FieldAttributes<In, COUNT> for T {
    fn attributes<Attr>(&self, p: In) -> <<Attr::Cons as Attributes>::Type as Uncons<COUNT>>::Uncons
    where
        Self: Fields<In, Attr::Cons>,
        Attr: Default + Cons,
        Attr::Cons: Attributes,
        <Attr::Cons as Attributes>::Type: Uncons<COUNT>,
    {
        self.fields(Attr::default().cons(), p).uncons()
    }
}

