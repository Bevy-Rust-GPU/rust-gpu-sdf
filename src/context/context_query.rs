use crate::prelude::Context;

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
    T: Context<'a, State, LHS> + ContextQuery<'a, Inner, RHS>,
    LHS: Clone + 'a,
{
    type Type = (
        LHS,
        <T as ContextQuery<'a, Inner, RHS>>::Type,
    );

    fn context_query(&'a self) -> Self::Type {
        (self.context().clone(), self.context_query())
    }
}

impl<'a, T> ContextQuery<'a, (), ()> for T {
    type Type = ();

    fn context_query(&'a self) -> Self::Type {
        ()
    }
}
