use crate::prelude::{AttributeRef, Context, Field};

/// Function associating an attribute stored in a context with a point in space.
///
/// API extension trait to provide input to `Field::field` via `Context::context`.
pub trait FieldContext<'a, Attr, Ctx, State>: Field<Attr>
where
    Attr: AttributeRef<'a>,
    Ctx: Context<State, &'a Attr::Input>,
{
    fn field_context(&self, ctx: Ctx) -> Attr::Output;
}

impl<'a, T, Attr, Ctx, State> FieldContext<'a, Attr, Ctx, State> for T
where
    Self: Field<Attr>,
    Attr: AttributeRef<'a>,
    Ctx: Context<State, &'a Attr::Input>,
{
    fn field_context(&self, ctx: Ctx) -> Attr::Output {
        self.field(ctx.context())
    }
}
