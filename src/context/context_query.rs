use crate::prelude::{Context, ContextPath};

/// Type-level data access to multiple fields
///
/// Extension trait over Context;
/// impls over (LHS, RHS) and (LHS, ()) with corresponding
/// (State, Inner) and (State, ()) `ContextPath` parameters.
///
/// This allows traversing a context several times with
/// distinct paths and returning a cons list of values
pub trait ContextQuery<'a, State, T> {
    type Type: 'a;

    fn context_query(&'a self) -> Self::Type;
}

impl<'a, T, LHS, RHS, State, Inner> ContextQuery<'a, (State, Inner), (LHS, RHS)> for T
where
    State: ContextPath,
    Inner: ContextPath,
    T: Context<'a, State, LHS> + ContextQuery<'a, Inner, RHS>,
{
    type Type = (
        &'a <T as Context<'a, State, LHS>>::Type,
        <T as ContextQuery<'a, Inner, RHS>>::Type,
    );

    fn context_query(&'a self) -> Self::Type {
        (self.context(), self.context_query())
    }
}

impl<'a, T, State, LHS> ContextQuery<'a, (State, ()), (LHS, ())> for T
where
    T: Context<'a, State, LHS>,
    State: ContextPath,
{
    type Type = (&'a <T as Context<'a, State, LHS>>::Type, ());

    fn context_query(&'a self) -> Self::Type {
        (self.context(), ())
    }
}
