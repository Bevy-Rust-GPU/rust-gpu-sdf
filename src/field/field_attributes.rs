//! Function associating several attribute values with a point in space.

use type_fields::cons::{Cons, Uncons};

use crate::prelude::{Attributes, Context, Fields, FieldsContext};

/// Function associating several attribute values with a point in space.
///
/// Extension trait of `Fields`;
/// `Cons`es the provided tuple, evaluates it via `FieldAttributesImpl`,
/// then returns the `Uncons` of the result.
pub trait FieldAttributes {
    fn attributes<Attr>(
        &self,
        p: <Attr::Cons as Attributes>::Input,
    ) -> <<Attr::Cons as Attributes>::Output as Uncons>::Uncons
    where
        Self: Fields<Attr::Cons>,
        Attr: Cons,
        Attr::Cons: Attributes,
        <Attr::Cons as Attributes>::Output: Uncons;
}

impl<T> FieldAttributes for T {
    fn attributes<Attr>(
        &self,
        p: <Attr::Cons as Attributes>::Input,
    ) -> <<Attr::Cons as Attributes>::Output as Uncons>::Uncons
    where
        Self: Fields<Attr::Cons>,
        Attr: Cons,
        Attr::Cons: Attributes,
        <Attr::Cons as Attributes>::Output: Uncons,
    {
        self.fields(p).uncons()
    }
}

/// Function associating several attribute values with a point in space.
///
/// Extension trait of `Fields`;
/// `Cons`es the provided tuple, evaluates it via `FieldAttributesImpl`,
/// then returns the `Uncons` of the result.
pub trait FieldAttributesContext<'a, Ctx, State> {
    fn attributes_context<Attr>(
        &'a self,
        ctx: &'a Ctx,
    ) -> <<Attr::Cons as Attributes>::Output as Uncons>::Uncons
    where
        Self: Fields<Attr::Cons>,
        Self: FieldsContext<'a, Ctx, State>,
        Ctx: Context<'a, State, <Attr::Cons as Attributes>::Input>,
        Attr: Cons,
        Attr::Cons: Attributes,
        <Attr::Cons as Attributes>::Input: Clone + 'a,
        <Attr::Cons as Attributes>::Output: Uncons;
}

impl<'a, T, Ctx, State> FieldAttributesContext<'a, Ctx, State> for T {
    fn attributes_context<Attr>(
        &'a self,
        ctx: &'a Ctx,
    ) -> <<Attr::Cons as Attributes>::Output as Uncons>::Uncons
    where
        Self: Fields<Attr::Cons>,
        Self: FieldsContext<'a, Ctx, State>,
        Ctx: Context<'a, State, <Attr::Cons as Attributes>::Input>,
        Attr: Cons,
        Attr::Cons: Attributes,
        <Attr::Cons as Attributes>::Input: Clone + 'a,
        <Attr::Cons as Attributes>::Output: Uncons,
    {
        self.fields_context(ctx).uncons()
    }
}
