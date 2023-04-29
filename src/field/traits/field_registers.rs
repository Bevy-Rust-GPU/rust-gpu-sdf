use crate::prelude::{AttributeRef, Registers, Field};

pub trait FieldRegisters<'a, Attr, Ctx, State>: Field<Attr>
where
    Attr: AttributeRef<'a>,
    Ctx: Registers<State, Attr::InputRef>,
{
    fn field_registers(&self, ctx: Ctx) -> Attr::Output;
}

impl<'a, T, Attr, Ctx, State> FieldRegisters<'a, Attr, Ctx, State> for T
where
    Self: Field<Attr>,
    Attr: AttributeRef<'a>,
    Ctx: Registers<State, Attr::InputRef, Type = Attr::InputRef>,
{
    fn field_registers(&self, ctx: Ctx) -> Attr::Output {
        self.field(&ctx.registers())
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use core::marker::PhantomData;

    use type_fields::t_funk::tlist::AsHListRef;

    use crate::prelude::{Attribute, Registers, Field, FieldRegisters};

    #[test]
    pub fn test_field_contexts() {
        let context = (1usize, 2.0, "three").as_hlist_ref();

        let (_float, (_int, ())) = Registers::<_, (&f32, (&usize, ()))>::registers(context);

        pub struct TestContexts;

        #[derive(Debug, Default, Copy, Clone)]
        pub struct AttrTestContexts<'a>(PhantomData<&'a ()>);

        impl<'a> Attribute for AttrTestContexts<'a> {
            type Input = (&'a f32, (&'a usize, ()));
            type Output = (usize, f32);
        }

        impl Field<AttrTestContexts<'_>> for TestContexts {
            fn field(
                &self,
                (float, (int, ())): &<AttrTestContexts as Attribute>::Input,
            ) -> <AttrTestContexts as Attribute>::Output {
                (**int, **float)
            }
        }

        let _out = TestContexts.field_registers(context);
    }
}
