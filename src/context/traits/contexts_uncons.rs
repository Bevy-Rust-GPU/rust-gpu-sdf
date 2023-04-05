use type_fields::cons::{Cons, Uncons};

use crate::prelude::Contexts;

/// Type-level data access to multiple fields
///
/// Extension trait over `Contexts`;
/// moves `Items` into the function position.
pub trait ContextsUncons<State, Items>: Contexts<State, Items::Cons>
where
    Items: Cons,
{
    fn contexts_uncons(self) -> <Items::Cons as Uncons>::Uncons;
}

impl<T, State, Items> ContextsUncons<State, Items> for T
where
    Self: Contexts<State, Items::Cons, Type = Items::Cons>,
    Items: Cons,
{
    fn contexts_uncons(self) -> <Items::Cons as Uncons>::Uncons {
        self.contexts().uncons()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use type_fields::cons::Cons;

    use crate::prelude::ContextsUncons;

    #[test]
    pub fn test_contexts_uncons() {
        let context = (1, 2.0, "three").cons();

        let (_int,) = ContextsUncons::<_, (usize,)>::contexts_uncons(0usize);
        let (_float,) = ContextsUncons::<_, (f32,)>::contexts_uncons(0.0);
        let (_string,) = ContextsUncons::<_, (&str,)>::contexts_uncons("hello");

        let (_string,) = ContextsUncons::<_, (&str,)>::contexts_uncons(context);
        let (_string, _float) = ContextsUncons::<_, (&str, f32)>::contexts_uncons(context);
        let (_string, _float, _int) =
            ContextsUncons::<_, (&str, f32, usize)>::contexts_uncons(context);
    }
}
