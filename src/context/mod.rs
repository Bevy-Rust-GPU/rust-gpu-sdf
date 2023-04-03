//! Type-level data access

pub mod context_item;
pub mod context_items;
pub mod context_path;
pub mod context_plural;
pub mod context_query;

use crate::prelude::{Car, Cdr};

/// Type-level data access
///
/// impls over `(LHS, RHS)` for evaluation of
/// arbitrary long cons lists.
pub trait Context<State, T>: Sized {
    fn context(&self) -> &T;
}

impl<LHS, RHS, Inner, T> Context<(Cdr, Inner), T> for (LHS, RHS)
where
    RHS: Context<Inner, T>,
{
    fn context(&self) -> &T {
        self.1.context()
    }
}

impl<LHS, RHS> Context<(Car, ()), LHS> for (LHS, RHS)
{
    fn context(&self) -> &LHS {
        &self.0
    }
}

/*
impl<'a, T> Context<'a, This, T> for T
where
    T: 'a,
{
    type Type = T;

    fn context(&self) -> &Self::Type {
        self
    }
}
*/

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use type_fields::cons::Cons;

    use crate::prelude::{ContextItem, ContextItems};

    #[test]
    pub fn test_context() {
        let context = (1, 2.0, "three").cons();

        /*
        let int = 0usize.item::<usize>();
        let float = 0.0.item::<f32>();
        let string = "hello".item::<&str>();
        */

        let int = context.item::<usize>();
        let float = context.item::<f32>();
        let string = context.item::<&str>();

        let (string, float, int) = context.context_items::<(&str, f32, usize)>();
    }
}
