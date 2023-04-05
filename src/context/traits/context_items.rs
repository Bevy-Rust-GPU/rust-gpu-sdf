use type_fields::cons::{Cons, Uncons};

use crate::prelude::Contexts;

/// Type-level data access to multiple fields
///
/// Extension trait over `Contexts`;
/// moves `Items` into the function position.
pub trait ContextItems<State>: Sized {
    fn context_items<Items>(self) -> <Self as Contexts<State, Items::Cons>>::Type
    where
        Items: Cons,
        Self: Contexts<State, Items::Cons>,
        <Self as Contexts<State, Items::Cons>>::Type: Uncons;
}

impl<T, State> ContextItems<State> for T {
    fn context_items<Items>(self) -> <Self as Contexts<State, Items::Cons>>::Type
    where
        Items: Cons,
        Self: Contexts<State, Items::Cons>,
    {
        self.contexts()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use type_fields::cons::Cons;

    use crate::prelude::ContextItems;

    #[test]
    pub fn test_context_items() {
        let context = (1, 2.0, "three").cons();

        let (_int, ()) = 0usize.context_items::<(usize,)>();
        let (_float, ()) = 0.0.context_items::<(f32,)>();
        let (_string, ()) = "hello".context_items::<(&str,)>();

        let (_string, ()) = context.context_items::<(&str,)>();
        let (_string, (_float, ())) = context.context_items::<(&str, f32)>();
        let (_string, (_float, (_int, ()))) = context.context_items::<(&str, f32, usize)>();
    }
}
