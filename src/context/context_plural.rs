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
    <Self as ContextQuery<'a, State, Items::Cons>>::Type: Uncons,
{
    type Plural: 'a;

    fn context_plural(&'a self) -> Self::Plural;
}

impl<'a, T, State, Items> ContextPlural<'a, State, Items> for T
where
    Items: Cons,
    T: ContextQuery<'a, State, Items::Cons>,
    <T as ContextQuery<'a, State, Items::Cons>>::Type: Uncons,
{
    type Plural = <<T as ContextQuery<'a, State, Items::Cons>>::Type as Uncons>::Uncons;

    fn context_plural(&'a self) -> Self::Plural {
        self.context_query().uncons()
    }
}
