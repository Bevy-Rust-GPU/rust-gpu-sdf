use crate::prelude::Context;

/// Type-level data access
///
/// Extension API trait over `Context`;
/// moves T into the function position.
pub trait ContextItem<'a, State>: Sized {
    fn item<Item>(&'a self) -> &'a Item
    where
        Self: Context<'a, State, Item>;
}

impl<'a, T, State> ContextItem<'a, State> for T {
    fn item<Item>(&'a self) -> &'a Item
    where
        T: Context<'a, State, Item>,
    {
        Context::<State, Item>::context(self)
    }
}
