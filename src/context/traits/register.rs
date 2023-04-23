use crate::prelude::{Car, Cdr, This};

/// Fetch `T` by type from a cons list.
pub trait Register<State, T>: Sized {
    fn register(self) -> T;
}

impl<LHS, RHS, Inner, T> Register<(Cdr, Inner), T> for (LHS, RHS)
where
    RHS: Register<Inner, T>,
{
    fn register(self) -> T {
        self.1.register()
    }
}

impl<LHS, RHS> Register<(Car, ()), LHS> for (LHS, RHS) {
    fn register(self) -> LHS {
        self.0
    }
}

impl<'a, T> Register<This, T> for T
where
    T: 'a,
{
    fn register(self) -> T {
        self
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use type_fields::cons::Cons;

    use crate::prelude::Register;

    #[test]
    pub fn test_context() {
        let context = (1, 2.0, "three").cons();

        let _int = 0usize.register();
        let _float = 0.0.register();
        let _string = "hello".register();

        let _int = Register::<_, usize>::register(context);
        let _float = Register::<_, f32>::register(context);
        let _string = Register::<_, &str>::register(context);
    }
}
