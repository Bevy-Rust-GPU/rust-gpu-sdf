//! Type-level data access

pub mod context_item;
pub mod context_items;
pub mod context_path;
pub mod context_plural;
pub mod context_query;

use crate::prelude::{Car, Cdr};

use self::context_path::{ContextPath, This};

/// Type-level data access
///
/// impls over `(LHS, RHS)` for evaluation of
/// arbitrary long cons lists.
pub trait Context<'a, State, T>
where
    State: ContextPath,
{
    type Type: 'a;

    fn context(&'a self) -> &'a Self::Type;
}

impl<'a, LHS, RHS, Inner, T> Context<'a, (Car, Inner), T> for (LHS, RHS)
where
    RHS: Context<'a, Inner, T>,
    Inner: ContextPath,
{
    type Type = RHS::Type;

    fn context(&'a self) -> &'a Self::Type {
        self.1.context()
    }
}

impl<'a, LHS, RHS> Context<'a, Cdr, LHS> for (LHS, RHS)
where
    LHS: 'a,
{
    type Type = LHS;

    fn context(&'a self) -> &'a Self::Type {
        &self.0
    }
}

impl Context<'_, Cdr, ()> for () {
    type Type = ();

    fn context(&self) -> &Self::Type {
        self
    }
}

impl<'a, T> Context<'a, This, T> for T
where
    T: 'a,
{
    type Type = T;

    fn context(&self) -> &Self::Type {
        self
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use type_fields::cons::Cons;

    use crate::prelude::{ContextItem, ContextItems};

    #[test]
    pub fn test_context() {
        let context = (1, 2.0, "three").cons();

        let int = 0usize.item::<usize>();
        let float = 0.0.item::<f32>();
        let string = "hello".item::<&str>();

        let int = context.item::<usize>();
        let float = context.item::<f32>();
        let string = context.item::<&str>();

        let (string, float, int) = context.context_items::<(&str, f32, usize)>();
    }
}
