use crate::prelude::{Car, Cdr, This};

/// Type-level data access
///
/// impls over `(LHS, RHS)` for evaluation of
/// arbitrary long cons lists.
pub trait Context<State, T>: Sized {
    fn context(self) -> T;
}

impl<LHS, RHS, Inner, T> Context<(Cdr, Inner), T> for (LHS, RHS)
where
    RHS: Context<Inner, T>,
{
    fn context(self) -> T {
        self.1.context()
    }
}

impl<LHS, RHS> Context<(Car, ()), LHS> for (LHS, RHS) {
    fn context(self) -> LHS {
        self.0
    }
}

impl<'a, T> Context<This, T> for T
where
    T: 'a,
{
    fn context(self) -> T {
        self
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use type_fields::cons::Cons;

    use crate::prelude::Context;

    #[test]
    pub fn test_context() {
        let context = (1, 2.0, "three").cons();

        let _int = 0usize.context();
        let _float = 0.0.context();
        let _string = "hello".context();

        let _int = Context::<_, usize>::context(context);
        let _float = Context::<_, f32>::context(context);
        let _string = Context::<_, &str>::context(context);
    }
}
