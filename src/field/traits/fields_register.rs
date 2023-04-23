use crate::prelude::{AttributesRef, Register, Fields};

/// Function associating an attribute value with a point in space.
///
/// API extension trait of `Fields`;
/// moves the `Attr` generic into the function position,
/// and obscures the `attr` parameter using `Attribute`'s `Default` constraint
pub trait FieldsRegister<'a, Ctx, State, Attr>: Fields<Attr>
where
    Attr: AttributesRef<'a>,
    Ctx: Register<State, &'a Attr::Input>,
{
    fn fields_register(&self, p: Ctx) -> Attr::Output;
}

impl<'a, T, Ctx, State, Attrs> FieldsRegister<'a, Ctx, State, Attrs> for T
where
    Self: Fields<Attrs>,
    Attrs: AttributesRef<'a>,
    Ctx: Register<State, &'a Attrs::Input>,
{
    fn fields_register(&self, ctx: Ctx) -> Attrs::Output {
        self.fields(ctx.register())
    }
}

