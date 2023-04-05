use type_fields::cons::{ConsRef, Uncons};

use crate::prelude::{ConsAttributes, Context, FieldsContext};

pub trait FieldAttributesContextCons<'a, Ctx, State>
where
    Ctx: ConsRef<'a>,
{
    fn attributes_context_cons<Attr>(&self, ctx: &'a Ctx) -> Attr::UnconsOutput
    where
        Self: FieldsContext<'a, Ctx::ConsRef, State, Attr::Cons>,
        Ctx::ConsRef: Context<State, &'a Attr::AttrInput>,
        Attr: ConsAttributes<'a>;
}

impl<'a, T, Ctx, State> FieldAttributesContextCons<'a, Ctx, State> for T
where
    Ctx: ConsRef<'a>,
{
    fn attributes_context_cons<Attr>(&self, ctx: &'a Ctx) -> Attr::UnconsOutput
    where
        Self: FieldsContext<'a, Ctx::ConsRef, State, Attr::Cons>,
        Ctx::ConsRef: Context<State, &'a Attr::AttrInput>,
        Attr: ConsAttributes<'a>,
    {
        self.fields_context(ctx.cons_ref()).uncons()
    }
}

