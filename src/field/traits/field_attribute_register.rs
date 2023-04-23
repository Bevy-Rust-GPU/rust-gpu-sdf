use crate::prelude::{AttributeRef, Register, FieldRegister};

/// Evalute the attribute `Attr` of a field function, drawing input from `Ctx`.
///
/// Moves `Attr` into the function position.
pub trait FieldAttributeRegister<'a, Ctx, State> {
    fn field_attribute_register<Attr>(&self, p: Ctx) -> Attr::Output
    where
        Self: FieldRegister<'a, Attr, Ctx, State>,
        Attr: AttributeRef<'a>,
        Ctx: Register<State, &'a Attr::Input>;
}

impl<'a, T, Ctx, State> FieldAttributeRegister<'a, Ctx, State> for T {
    fn field_attribute_register<Attr>(&self, ctx: Ctx) -> Attr::Output
    where
        T: FieldRegister<'a, Attr, Ctx, State>,
        Attr: AttributeRef<'a>,
        Ctx: Register<State, &'a Attr::Input>,
    {
        self.field_register(ctx)
    }
}

