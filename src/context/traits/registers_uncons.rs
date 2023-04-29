use type_fields::t_funk::{hlist::ToTList, tlist::ToHList};

use crate::prelude::Registers;

/// Fetch multiple items by type from a cons list and uncons them before return.
pub trait RegistersUncons<State, Items>: Registers<State, Items::HList>
where
    Items: ToHList,
{
    fn registers_uncons(self) -> <Items::HList as ToTList>::TList;
}

impl<T, State, Items> RegistersUncons<State, Items> for T
where
    Self: Registers<State, Items::HList, Type = Items::HList>,
    Items: ToHList,
{
    fn registers_uncons(self) -> <Items::HList as ToTList>::TList {
        self.registers().to_tlist()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use type_fields::t_funk::tlist::ToHList;

    use crate::prelude::RegistersUncons;

    #[test]
    pub fn test_contexts_uncons() {
        let context = (1, 2.0, "three").to_hlist();

        let (_int,) = RegistersUncons::<_, (usize,)>::registers_uncons(0usize);
        let (_float,) = RegistersUncons::<_, (f32,)>::registers_uncons(0.0);
        let (_string,) = RegistersUncons::<_, (&str,)>::registers_uncons("hello");

        let (_string,) = RegistersUncons::<_, (&str,)>::registers_uncons(context);
        let (_string, _float) = RegistersUncons::<_, (&str, f32)>::registers_uncons(context);
        let (_string, _float, _int) =
            RegistersUncons::<_, (&str, f32, usize)>::registers_uncons(context);
    }
}
