use type_fields::t_funk::{tlist::AsHListRef, hlist::ToTList};

use crate::prelude::{ConsAttributes, FieldsRegister, FieldsRegisters, Register, Registers};

/// Evalute multiple attributes of a field function,
/// drawing input from `Ctx` and `Uncons`ing the result.
///
/// Moves `Attrs` into the function position.
pub trait FieldAttributesRegisterCons<'a, Ctx, State>
where
    Ctx: AsHListRef,
{
    fn field_attributes_register_cons<Attrs>(&self, ctx: &'a Ctx) -> Attrs::UnconsOutput
    where
        Self: FieldsRegister<'a, Ctx::HListRef<'a>, State, Attrs::HList>,
        Ctx::HListRef<'a>: Register<State, &'a Attrs::AttrInput>,
        Attrs: ConsAttributes<'a>;
}

impl<'a, T, Ctx, State> FieldAttributesRegisterCons<'a, Ctx, State> for T
where
    Ctx: AsHListRef,
{
    fn field_attributes_register_cons<Attr>(&self, ctx: &'a Ctx) -> Attr::UnconsOutput
    where
        Self: FieldsRegister<'a, Ctx::HListRef<'a>, State, Attr::HList>,
        Ctx::HListRef<'a>: Register<State, &'a Attr::AttrInput>,
        Attr: ConsAttributes<'a>,
    {
        self.fields_register(ctx.as_hlist_ref()).to_tlist()
    }
}

/// Evalute multiple attributes of a field function,
/// drawing multiple inputs from `Ctx` and `Uncons`ing the result.
///
/// Moves `Attrs` into the function position.
pub trait FieldAttributesRegistersCons<'a, Ctx, State>
where
    Ctx: AsHListRef,
{
    fn field_attributes_registers_cons<Attrs>(&self, ctx: &'a Ctx) -> Attrs::UnconsOutput
    where
        Self: FieldsRegisters<'a, Ctx::HListRef<'a>, State, Attrs::HList>,
        Attrs: ConsAttributes<'a>,
        Ctx::HListRef<'a>: Registers<State, Attrs::AttrInput, Type = Attrs::AttrInput>;
}

impl<'a, T, Ctx, State> FieldAttributesRegistersCons<'a, Ctx, State> for T
where
    Ctx: AsHListRef,
{
    fn field_attributes_registers_cons<Attrs>(&self, ctx: &'a Ctx) -> Attrs::UnconsOutput
    where
        Self: FieldsRegisters<'a, Ctx::HListRef<'a>, State, Attrs::HList>,
        Attrs: ConsAttributes<'a>,
        Ctx::HListRef<'a>: Registers<State, Attrs::AttrInput, Type = Attrs::AttrInput>,
    {
        self.fields_registers(ctx.as_hlist_ref()).to_tlist()
    }
}
