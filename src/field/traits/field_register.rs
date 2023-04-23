use crate::prelude::{AttributeRef, Register, Field};

/// Evalute the attribute `Attr` of a field function, drawing input from `Ctx`.
pub trait FieldRegister<'a, Attr, Ctx, State>: Field<Attr>
where
    Attr: AttributeRef<'a>,
    Ctx: Register<State, &'a Attr::Input>,
{
    fn field_register(&self, ctx: Ctx) -> Attr::Output;
}

impl<'a, T, Attr, Ctx, State> FieldRegister<'a, Attr, Ctx, State> for T
where
    Self: Field<Attr>,
    Attr: AttributeRef<'a>,
    Ctx: Register<State, &'a Attr::Input>,
{
    fn field_register(&self, ctx: Ctx) -> Attr::Output {
        self.field(ctx.register())
    }
}

