use type_fields::cons::{Cons, Uncons};

use crate::prelude::Contexts;

use super::contexts_uncons::ContextsUncons;

/// Type-level data access to multiple fields
///
/// Extension trait over `Contexts`;
/// moves `Items` into the function position.
pub trait ContextItemsUncons<State>: Sized {
    fn context_items_uncons<Items>(
        self,
    ) -> <<Self as Contexts<State, Items::Cons>>::Type as Uncons>::Uncons
    where
        Self: Contexts<State, Items::Cons, Type = Items::Cons>,
        Items: Cons,
        Self::Type: Uncons;
}

impl<T, State> ContextItemsUncons<State> for T {
    fn context_items_uncons<Items>(
        self,
    ) -> <<Self as Contexts<State, Items::Cons>>::Type as Uncons>::Uncons
    where
        Self: Contexts<State, Items::Cons, Type = Items::Cons>,
        Items: Cons,
        <Self as Contexts<State, Items::Cons>>::Type: Uncons,
    {
        ContextsUncons::<State, Items>::contexts_uncons(self)
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use type_fields::cons::Cons;

    use crate::prelude::ContextItemsUncons;

    #[test]
    pub fn test_context_items() {
        let context = (1, 2.0, "three").cons();

        let (_int,) = 0usize.context_items_uncons::<(usize,)>();
        let (_float,) = 0.0.context_items_uncons::<(f32,)>();
        let (_string,) = "hello".context_items_uncons::<(&str,)>();

        let (_string,) = context.context_items_uncons::<(&str,)>();
        let (_string, _float) = context.context_items_uncons::<(&str, f32)>();
        let (_string, _float, _int) = context.context_items_uncons::<(&str, f32, usize)>();
    }
}

