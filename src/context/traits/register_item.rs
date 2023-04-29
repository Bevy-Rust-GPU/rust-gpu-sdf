use crate::prelude::Register;

/// Fetch `T` by type from a cons list.
///
/// Moves `T` into the function position.
pub trait RegisterItem<State>: Sized {
    fn item<Item>(self) -> Item
    where
        Self: Register<State, Item>;
}

impl<T, State> RegisterItem<State> for T {
    fn item<Item>(self) -> Item
    where
        T: Register<State, Item>,
    {
        self.register()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use type_fields::t_funk::tlist::ToHList;

    use crate::prelude::RegisterItem;

    #[test]
    pub fn test_context_item() {
        let context = (1, 2.0, "three").to_hlist();

        let _int = 0usize.item::<usize>();
        let _float = 0.0.item::<f32>();
        let _string = "hello".item::<&str>();

        let _int = context.item::<usize>();
        let _float = context.item::<f32>();
        let _string = context.item::<&str>();
    }
}
