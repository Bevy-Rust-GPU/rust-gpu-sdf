use crate::prelude::{AttributeRef, Contexts, Field};

pub trait FieldContexts<'a, Attr, Ctx, State>: Field<Attr>
where
    Attr: AttributeRef<'a>,
    Ctx: Contexts<State, Attr::InputRef>,
{
    fn field_contexts(&self, ctx: Ctx) -> Attr::Output;
}

impl<'a, T, Attr, Ctx, State> FieldContexts<'a, Attr, Ctx, State> for T
where
    Self: Field<Attr>,
    Attr: AttributeRef<'a>,
    Ctx: Contexts<State, Attr::InputRef, Type = Attr::InputRef>,
{
    fn field_contexts(&self, ctx: Ctx) -> Attr::Output {
        self.field(&ctx.contexts())
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use core::marker::PhantomData;

    use type_fields::cons::ConsRef;

    use crate::prelude::{Attribute, Contexts, Field, FieldContexts};

    #[test]
    pub fn test_field_contexts() {
        let context = (1usize, 2.0, "three").cons_ref();

        let (_float, (_int, ())) = Contexts::<_, (&f32, (&usize, ()))>::contexts(context);

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

        let _out = TestContexts.field_contexts(context);
    }
}
