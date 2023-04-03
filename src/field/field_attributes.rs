//! Function associating several attribute values with a point in space.

use type_fields::cons::{Cons, ConsRef, Uncons};

use crate::prelude::{Attributes, Context, Fields, FieldsContext};

/// Function associating several attribute values with a point in space.
///
/// Extension trait of `Fields`;
/// `Cons`es the provided tuple, evaluates it via `FieldAttributesImpl`,
/// then returns the `Uncons` of the result.
pub trait FieldAttributes {
    fn attributes<Attr>(
        &self,
        input: &<Attr::Cons as Attributes>::Input,
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
        input: &<Attr::Cons as Attributes>::Input,
    ) -> <<Attr::Cons as Attributes>::Output as Uncons>::Uncons
    where
        Self: Fields<Attr::Cons>,
        Attr: Cons,
        Attr::Cons: Attributes,
        <Attr::Cons as Attributes>::Output: Uncons,
    {
        self.fields(input).uncons()
    }
}

/// Function associating several attribute values with a point in space.
///
/// Extension trait of `Fields`;
/// `Cons`es the provided tuple, evaluates it via `FieldAttributesImpl`,
/// then returns the `Uncons` of the result.
pub trait FieldAttributesContext<'a, Ctx, State> {
    fn attributes_context<Attr>(
        &self,
        ctx: Ctx,
    ) -> <<Attr::Cons as Attributes>::Output as Uncons>::Uncons
    where
        Self: Fields<Attr::Cons>,
        Self: FieldsContext<'a, Ctx, State>,
        Ctx: Context<State, &'a <Attr::Cons as Attributes>::Input>,
        Attr: Cons,
        Attr::Cons: Attributes,
        <Attr::Cons as Attributes>::Input: 'a,
        <Attr::Cons as Attributes>::Output: Uncons;
}

impl<'a, T, Ctx, State> FieldAttributesContext<'a, Ctx, State> for T {
    fn attributes_context<Attr>(
        &self,
        ctx: Ctx,
    ) -> <<Attr::Cons as Attributes>::Output as Uncons>::Uncons
    where
        Self: Fields<Attr::Cons>,
        Self: FieldsContext<'a, Ctx, State>,
        Ctx: Context<State, &'a <Attr::Cons as Attributes>::Input>,
        Attr: Cons,
        Attr::Cons: Attributes,
        <Attr::Cons as Attributes>::Input: 'a,
        <Attr::Cons as Attributes>::Output: Uncons,
    {
        self.fields_context(ctx).uncons()
    }
}

pub trait FieldAttributesContextCons<'a, Ctx, State>
where
    Ctx: ConsRef<'a>,
{
    fn attributes_context_cons<Attr>(
        &self,
        ctx: &'a Ctx,
    ) -> <<Attr::Cons as Attributes>::Output as Uncons>::Uncons
    where
        Self: Fields<Attr::Cons>,
        Self: FieldsContext<'a, Ctx::ConsRef, State>,
        Ctx::ConsRef: Context<State, &'a <<Attr as Cons>::Cons as Attributes>::Input>,
        Attr: Cons,
        Attr::Cons: Attributes,
        <Attr::Cons as Attributes>::Input: 'a,
        <Attr::Cons as Attributes>::Output: Uncons,
        Self: Fields<Attr::Cons>;
}

impl<'a, T, Ctx, State> FieldAttributesContextCons<'a, Ctx, State> for T
where
    Ctx: ConsRef<'a>,
{
    fn attributes_context_cons<Attr>(
        &self,
        ctx: &'a Ctx,
    ) -> <<Attr::Cons as Attributes>::Output as Uncons>::Uncons
    where
        Self: Fields<Attr::Cons>,
        Self: FieldsContext<'a, Ctx::ConsRef, State>,
        Ctx::ConsRef: Context<State, &'a <<Attr as Cons>::Cons as Attributes>::Input>,
        Attr: Cons,
        Attr::Cons: Attributes,
        <Attr::Cons as Attributes>::Input: 'a,
        <Attr::Cons as Attributes>::Output: Uncons,
    {
        self.fields_context(ctx.cons_ref()).uncons()
    }
}
