use crate::prelude::{AttributesRef, Contexts, Fields};

pub trait FieldsContexts<'a, Attr, Ctx, State>: Fields<Attr>
where
    Attr: AttributesRef<'a>,
    Ctx: Contexts<State, Attr::InputRef>,
{
    fn fields_contexts(&self, ctx: Ctx) -> Attr::Output;
}

impl<'a, T, Attr, Ctx, State> FieldsContexts<'a, Attr, Ctx, State> for T
where
    Self: Fields<Attr>,
    Attr: AttributesRef<'a>,
    Ctx: Contexts<State, Attr::InputRef, Type = Attr::InputRef>,
{
    fn fields_contexts(&self, ctx: Ctx) -> Attr::Output {
        self.fields(&ctx.contexts())
    }
}
