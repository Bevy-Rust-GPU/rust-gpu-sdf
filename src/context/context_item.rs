use crate::prelude::{Context, ContextPath};

/// Type-level data access
///
/// Extension API trait over `Context`;
/// moves T into the function position.
pub trait ContextItem<'a, State>
where
    State: ContextPath,
{
    fn item<T>(&'a self) -> &'a Self::Type
    where
        Self: Context<'a, State, T>;
}

impl<'a, T, State> ContextItem<'a, State> for T
where
    State: ContextPath,
{
    fn item<Item>(&'a self) -> &'a T::Type
    where
        T: Context<'a, State, Item>,
    {
        Context::<State, Item>::context(self)
    }
}
