use crate::prelude::Context;

/// Type-level data access
///
/// Extension API trait over `Context`;
/// moves T into the function position.
pub trait ContextItem<State>: Sized {
    fn item<Item>(self) -> Item
    where
        Self: Context<State, Item>;
}

impl<T, State> ContextItem<State> for T {
    fn item<Item>(self) -> Item
    where
        T: Context<State, Item>,
    {
        self.context()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use type_fields::cons::Cons;

    use crate::prelude::ContextItem;

    #[test]
    pub fn test_context_item() {
        let context = (1, 2.0, "three").cons();

        let _int = 0usize.item::<usize>();
        let _float = 0.0.item::<f32>();
        let _string = "hello".item::<&str>();

        let _int = context.item::<usize>();
        let _float = context.item::<f32>();
        let _string = context.item::<&str>();
    }
}
