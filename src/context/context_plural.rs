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
    type Plural;

    fn context_plural(&'a self) -> Self::Plural;
}

impl<'a, T, State, Items> ContextPlural<'a, State, Items> for T
where
    Self: ContextQuery<'a, State, Items::Cons>,
    Items: Cons,
    Self::Type: Uncons,
{
    type Plural = <Self::Type as Uncons>::Uncons;

    fn context_plural(&'a self) -> Self::Plural {
        self.context_query().uncons()
    }
}
