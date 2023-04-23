use type_fields::t_funk::hlist::ToTList;

use crate::prelude::{ConsAttributes, FieldsRegister, Register};

/// Evalute multiple attributes of a field function, drawing input from `Ctx`.
///
/// Moves `Attrs` into the function position.
pub trait FieldAttributesRegister<'a, Ctx, State> {
    fn field_attributes_register<Attr>(&self, ctx: Ctx) -> Attr::UnconsOutput
    where
        Self: FieldsRegister<'a, Ctx, State, Attr::HList>,
        Ctx: Register<State, &'a Attr::AttrInput>,
        Attr: ConsAttributes<'a>;
}

impl<'a, T, Ctx, State> FieldAttributesRegister<'a, Ctx, State> for T {
    fn field_attributes_register<Attr>(&self, ctx: Ctx) -> Attr::UnconsOutput
    where
        Self: FieldsRegister<'a, Ctx, State, Attr::HList>,
        Ctx: Register<State, &'a Attr::AttrInput>,
        Attr: ConsAttributes<'a>,
    {
        self.fields_register(ctx).to_tlist()
    }
}
