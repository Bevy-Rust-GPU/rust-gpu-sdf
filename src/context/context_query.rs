use crate::prelude::Context;

/// Type-level data access to multiple fields
///
/// Extension trait over Context;
/// impls over (LHS, RHS) and (LHS, ()) with corresponding
/// (State, Inner) and (State, ()) `ContextPath` parameters.
///
/// This allows traversing a context several times with
/// distinct paths and returning a cons list of values
pub trait ContextQuery<State, T> {
    type Type;

    fn context_query(self) -> Self::Type;
}

impl<T, LHS, RHS, State, Inner> ContextQuery<(State, Inner), (LHS, RHS)> for T
where
    T: Clone + Context<State, LHS> + ContextQuery<Inner, RHS>,
{
    type Type = (
        LHS,
        <T as ContextQuery<Inner, RHS>>::Type,
    );

    fn context_query(self) -> Self::Type {
        (self.clone().context(), self.context_query())
    }
}

impl<T> ContextQuery<(), ()> for T {
    type Type = ();

    fn context_query(self) -> Self::Type {
        ()
    }
}
