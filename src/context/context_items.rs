use type_fields::cons::Cons;

use super::context_plural::ContextPlural;

/// Type-level data access to multiple fields
///
/// Extension trait over `ContextPlural`;
/// moves `Items` into the function position.
pub trait ContextItems<State>: Sized {
    fn context_items<'a, Items>(&'a self) -> <Self as ContextPlural<'a, State, Items>>::Plural
    where
        Items: Cons,
        Self: ContextPlural<'a, State, Items>;
}

impl<T, State> ContextItems<State> for T {
    fn context_items<'a, Items>(&'a self) -> <Self as ContextPlural<'a, State, Items>>::Plural
    where
        Items: Cons,
        Self: ContextPlural<'a, State, Items>,
    {
        self.context_plural()
    }
}
