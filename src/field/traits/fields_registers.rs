use crate::prelude::{AttributesRef, Registers, Fields};

/// Evalute multiple attributes of a field function, drawing input from `Ctx`.
pub trait FieldsRegisters<'a, Ctx, State, Attrs>: Fields<Attrs>
where
    Attrs: AttributesRef<'a>,
    Ctx: Registers<State, Attrs::InputRef>,
{
    fn fields_registers(&self, ctx: Ctx) -> Attrs::Output;
}

impl<'a, T, Attrs, Ctx, State> FieldsRegisters<'a, Ctx, State, Attrs> for T
where
    Self: Fields<Attrs>,
    Attrs: AttributesRef<'a>,
    Ctx: Registers<State, Attrs::InputRef, Type = Attrs::InputRef>,
{
    fn fields_registers(&self, ctx: Ctx) -> Attrs::Output {
        self.fields(&ctx.registers())
    }
}
