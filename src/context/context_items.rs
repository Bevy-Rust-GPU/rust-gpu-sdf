use crate::prelude::ContextQuery;
use type_fields::cons::{Cons, Uncons};

use super::context_plural::ContextPlural;

/// Type-level data access to multiple fields
///
/// Extension trait over `ContextPlural`;
/// moves `Items` into the function position.
pub trait ContextItems<State>: Sized {
    fn context_items<'a, Items>(&'a self) -> <Self as ContextPlural<'a, State, Items>>::Plural
    where
        Items: Cons,
        Self: ContextPlural<'a, State, Items>,
        <Self as ContextQuery<'a, State, Items::Cons>>::Type: Uncons;
}

impl<T, State> ContextItems<State> for T {
    fn context_items<'a, Items>(&'a self) -> <Self as ContextPlural<'a, State, Items>>::Plural
    where
        Items: Cons,
        Self: ContextPlural<'a, State, Items>,
        <Self as ContextQuery<'a, State, Items::Cons>>::Type: Uncons,
    {
        self.context_plural()
    }
}
