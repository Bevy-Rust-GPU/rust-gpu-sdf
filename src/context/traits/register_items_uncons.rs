use type_fields::t_funk::{hlist::ToTList, tlist::ToHList};

use crate::prelude::Registers;

use super::registers_uncons::RegistersUncons;

/// Fetch multiple items by type from a cons list and uncons them before return.
///
/// Moves `Items` into the function position.
pub trait RegisterItemsUncons<State>: Sized {
    fn context_items_uncons<Items>(
        self,
    ) -> <<Self as Registers<State, Items::HList>>::Type as ToTList>::TList
    where
        Self: Registers<State, Items::HList, Type = Items::HList>,
        Items: ToHList,
        Self::Type: ToTList;
}

impl<T, State> RegisterItemsUncons<State> for T {
    fn context_items_uncons<Items>(
        self,
    ) -> <<Self as Registers<State, Items::HList>>::Type as ToTList>::TList
    where
        Self: Registers<State, Items::HList, Type = Items::HList>,
        Items: ToHList,
        <Self as Registers<State, Items::HList>>::Type: ToTList,
    {
        RegistersUncons::<State, Items>::registers_uncons(self)
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use type_fields::t_funk::tlist::ToHList;

    use crate::prelude::RegisterItemsUncons;

    #[test]
    pub fn test_context_items() {
        let context = (1, 2.0, "three").to_hlist();

        let (_int,) = 0usize.context_items_uncons::<(usize,)>();
        let (_float,) = 0.0.context_items_uncons::<(f32,)>();
        let (_string,) = "hello".context_items_uncons::<(&str,)>();

        let (_string,) = context.context_items_uncons::<(&str,)>();
        let (_string, _float) = context.context_items_uncons::<(&str, f32)>();
        let (_string, _float, _int) = context.context_items_uncons::<(&str, f32, usize)>();
    }
}
