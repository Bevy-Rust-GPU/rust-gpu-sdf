use type_fields::cons::{Cons, Uncons};

use crate::prelude::{AttributeRef, ContextsUncons, Field};

pub trait FieldContextsUncons<'a, Attr, Ctx, State>: Field<Attr>
where
    Attr: AttributeRef<'a>,
    Attr::InputRef: Cons,
    <Attr::InputRef as Cons>::Cons: Uncons<Uncons = Attr::InputRef>,
    Ctx: ContextsUncons<State, Attr::InputRef, Type = <Attr::InputRef as Cons>::Cons>,
{
    fn field_contexts_uncons(&self, ctx: Ctx) -> Attr::Output;
}

impl<'a, T, Attr, Ctx, State> FieldContextsUncons<'a, Attr, Ctx, State> for T
where
    Self: Field<Attr>,
    Attr: AttributeRef<'a>,
    Attr::InputRef: Cons,
    <Attr::InputRef as Cons>::Cons: Uncons<Uncons = Attr::InputRef>,
    Ctx: ContextsUncons<State, Attr::InputRef, Type = <Attr::InputRef as Cons>::Cons>,
{
    fn field_contexts_uncons(&self, ctx: Ctx) -> Attr::Output {
        self.field(&ctx.contexts_uncons())
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use core::marker::PhantomData;

    use type_fields::cons::ConsRef;

    use crate::prelude::{Attribute, ContextsUncons, Field, FieldContextsUncons};

    #[test]
    pub fn test_field_contexts_uncons() {
        let context = (1usize, 2.0, "three").cons_ref();

        let (_float, _int) = ContextsUncons::<_, (&f32, &usize)>::contexts_uncons(context);

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

        let _out = TestContextsUncons.field_contexts_uncons(context);
    }
}
