use crate::prelude::Context;

/// Type-level data access
///
/// Extension API trait over `Context`;
/// moves T into the function position.
pub trait ContextItem<State>: Sized {
    fn item<Item>(&self) -> &Item
    where
        Self: Context<State, Item>;
}

impl<T, State> ContextItem<State> for T {
    fn item<Item>(&self) -> &Item
    where
        T: Context<State, Item>,
    {
        Context::<State, Item>::context(self)
    }
}
