use type_fields::cons::{Cons, Uncons};

use crate::prelude::ContextQuery;

/// Type-level data access to multiple fields
///
/// Extension trait over `ContextQuery`;
/// queries using the `Cons`ed version of `Self`,
/// and `Uncons`es the result before return.
pub trait ContextPlural<'a, State, Items>: ContextQuery<'a, State, Items::Cons>
where
    Items: Cons,
{
    type Plural: 'a;

    fn context_plural(&'a self) -> Self::Plural;
}

impl<'a, T, State, Items> ContextPlural<'a, State, Items> for T
where
    Items: Cons,
    T: ContextQuery<'a, State, Items::Cons, Type = Items::Cons>,
    <Items::Cons as Uncons>::Uncons: 'a,
{
    type Plural = <Items::Cons as Uncons>::Uncons;

    fn context_plural(&'a self) -> Self::Plural {
        self.context_query().uncons()
    }
}
