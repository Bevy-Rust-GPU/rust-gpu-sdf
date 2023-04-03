use type_fields::cons::{Cons, Uncons};

use crate::prelude::ContextQuery;

/// Type-level data access to multiple fields
///
/// Extension trait over `ContextQuery`;
/// queries using the `Cons`ed version of `Self`,
/// and `Uncons`es the result before return.
pub trait ContextPlural<State, Items>: ContextQuery<State, Items::Cons>
where
    Items: Cons,
{
    type Plural;

    fn context_plural(self) -> Self::Plural;
}

impl<T, State, Items> ContextPlural<State, Items> for T
where
    Self: ContextQuery<State, Items::Cons>,
    Items: Cons,
    Self::Type: Uncons,
{
    type Plural = <Self::Type as Uncons>::Uncons;

    fn context_plural(self) -> Self::Plural {
        self.context_query().uncons()
    }
}
