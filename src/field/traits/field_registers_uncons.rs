use type_fields::t_funk::{hlist::ToTList, tlist::ToHList};

use crate::prelude::{AttributeRef, Field, RegistersUncons};

pub trait FieldRegistersUncons<'a, Attr, Ctx, State>: Field<Attr>
where
    Attr: AttributeRef<'a>,
    Attr::InputRef: ToHList,
    <Attr::InputRef as ToHList>::HList: ToTList<TList = Attr::InputRef>,
    Ctx: RegistersUncons<State, Attr::InputRef, Type = <Attr::InputRef as ToHList>::HList>,
{
    fn field_registers_uncons(&self, ctx: Ctx) -> Attr::Output;
}

impl<'a, T, Attr, Ctx, State> FieldRegistersUncons<'a, Attr, Ctx, State> for T
where
    Self: Field<Attr>,
    Attr: AttributeRef<'a>,
    Attr::InputRef: ToHList,
    <Attr::InputRef as ToHList>::HList: ToTList<TList = Attr::InputRef>,
    Ctx: RegistersUncons<State, Attr::InputRef, Type = <Attr::InputRef as ToHList>::HList>,
{
    fn field_registers_uncons(&self, ctx: Ctx) -> Attr::Output {
        self.field(&ctx.registers_uncons())
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use core::marker::PhantomData;

    use type_fields::t_funk::tlist::AsHListRef;

    use crate::prelude::{Attribute, Field, FieldRegistersUncons, RegistersUncons};

    #[test]
    pub fn test_field_contexts_uncons() {
        let context = (1usize, 2.0, "three").as_hlist_ref();

        let (_float, _int) = RegistersUncons::<_, (&f32, &usize)>::registers_uncons(context);

        pub struct TestContextsUncons;

        #[derive(Debug, Default, Copy, Clone)]
        pub struct AttrTestContextsUncons<'a>(PhantomData<&'a ()>);

        impl<'a> Attribute for AttrTestContextsUncons<'a> {
            type Input = (&'a f32, &'a usize);
            type Output = (usize, f32);
        }

        impl Field<AttrTestContextsUncons<'_>> for TestContextsUncons {
            fn field(
                &self,
                (float, int): &<AttrTestContextsUncons as Attribute>::Input,
            ) -> <AttrTestContextsUncons as Attribute>::Output {
                (**int, **float)
            }
        }

        let _out = TestContextsUncons.field_registers_uncons(context);
    }
}
