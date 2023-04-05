use crate::prelude::{Context, This};

/// Type-level data access to multiple fields
///
/// Extension trait over Context;
/// impls over (LHS, RHS) and (LHS, ()) with corresponding
/// (State, Inner) and (State, ()) `ContextPath` parameters.
///
/// This allows traversing a context several times with
/// distinct paths and returning a cons list of values
pub trait Contexts<State, T> {
    type Type;

    fn contexts(self) -> Self::Type;
}

impl<T, LHS, RHS, LState, RState> Contexts<(LState, RState), (LHS, RHS)> for T
where
    T: Clone + Context<LState, LHS> + Contexts<RState, RHS>,
{
    type Type = (LHS, <T as Contexts<RState, RHS>>::Type);

    fn contexts(self) -> Self::Type {
        (self.clone().context(), self.contexts())
    }
}

impl<T> Contexts<This, T> for T {
    type Type = T;

    fn contexts(self) -> Self::Type {
        self
    }
}

impl<T> Contexts<(), ()> for T {
    type Type = ();

    fn contexts(self) -> Self::Type {
        ()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use type_fields::cons::Cons;

    use crate::prelude::Contexts;

    #[test]
    pub fn test_contexts() {
        let context = (1, 2.0, "three").cons();

        /*
        let _int = 0usize.contexts();
        let _float = 0.0.contexts();
        let _string = "hello".contexts();
        */

        let _int = Contexts::<_, (usize, ())>::contexts(0usize);
        let _float = Contexts::<_, (f32, ())>::contexts(0.0);
        let _string = Contexts::<_, (&str, ())>::contexts("hello");

        let (string, (float, (int, ()))) =
            Contexts::<_, (&str, (f32, (usize, ())))>::contexts(context);
    }
}
