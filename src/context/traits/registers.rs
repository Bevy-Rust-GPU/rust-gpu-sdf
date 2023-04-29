use crate::prelude::{Register, This};

/// Fetch multiple items by type from a cons list.
pub trait Registers<State, T> {
    type Type;

    fn registers(self) -> Self::Type;
}

impl<T, LHS, RHS, LState, RState> Registers<(LState, RState), (LHS, RHS)> for T
where
    T: Clone + Register<LState, LHS> + Registers<RState, RHS>,
{
    type Type = (LHS, <T as Registers<RState, RHS>>::Type);

    fn registers(self) -> Self::Type {
        (self.clone().register(), self.registers())
    }
}

impl<T> Registers<This, T> for T {
    type Type = T;

    fn registers(self) -> Self::Type {
        self
    }
}

impl<T> Registers<(), ()> for T {
    type Type = ();

    fn registers(self) -> Self::Type {
        ()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use type_fields::t_funk::tlist::ToHList;

    use crate::prelude::Registers;

    #[test]
    pub fn test_contexts() {
        let context = (1, 2.0, "three").to_hlist();

        /*
        let _int = 0usize.contexts();
        let _float = 0.0.contexts();
        let _string = "hello".contexts();
        */

        let _int = Registers::<_, (usize, ())>::registers(0usize);
        let _float = Registers::<_, (f32, ())>::registers(0.0);
        let _string = Registers::<_, (&str, ())>::registers("hello");

        let (_string, (_float, (_int, ()))) =
            Registers::<_, (&str, (f32, (usize, ())))>::registers(context);
    }
}
