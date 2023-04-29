use type_fields::t_funk::{hlist::ToTList, tlist::ToHList};

use crate::prelude::Registers;

/// Fetch multiple items by type from a cons list.
///
/// Moves `Items` into the function position.
pub trait RegisterItems<State>: Sized {
    fn context_items<Items>(self) -> <Self as Registers<State, Items::HList>>::Type
    where
        Items: ToHList,
        Self: Registers<State, Items::HList>,
        <Self as Registers<State, Items::HList>>::Type: ToTList;
}

impl<T, State> RegisterItems<State> for T {
    fn context_items<Items>(self) -> <Self as Registers<State, Items::HList>>::Type
    where
        Items: ToHList,
        Self: Registers<State, Items::HList>,
    {
        self.registers()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use type_fields::t_funk::tlist::ToHList;

    use crate::prelude::RegisterItems;

    #[test]
    pub fn test_context_items() {
        let context = (1, 2.0, "three").to_hlist();

        let (_int, ()) = 0usize.context_items::<(usize,)>();
        let (_float, ()) = 0.0.context_items::<(f32,)>();
        let (_string, ()) = "hello".context_items::<(&str,)>();

        let (_string, ()) = context.context_items::<(&str,)>();
        let (_string, (_float, ())) = context.context_items::<(&str, f32)>();
        let (_string, (_float, (_int, ()))) = context.context_items::<(&str, f32, usize)>();
    }
}
