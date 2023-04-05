use type_fields::cons::{Cons, Uncons};

use crate::prelude::{AttributesRef, ContextsUncons, Fields};

pub trait FieldsContextsUncons<'a, Attr, Ctx, State>: Fields<Attr>
where
    Attr: AttributesRef<'a>,
    Attr::InputRef: Cons,
    <Attr::InputRef as Cons>::Cons: Uncons<Uncons = Attr::InputRef>,
    Ctx: ContextsUncons<State, Attr::InputRef, Type = <Attr::InputRef as Cons>::Cons>,
{
    fn fields_contexts_uncons(&self, ctx: Ctx) -> Attr::Output;
}

impl<'a, T, Attr, Ctx, State> FieldsContextsUncons<'a, Attr, Ctx, State> for T
where
    Self: Fields<Attr>,
    Attr: AttributesRef<'a>,
    Attr::InputRef: Cons,
    <Attr::InputRef as Cons>::Cons: Uncons<Uncons = Attr::InputRef>,
    Ctx: ContextsUncons<State, Attr::InputRef, Type = <Attr::InputRef as Cons>::Cons>,
{
    fn fields_contexts_uncons(&self, ctx: Ctx) -> Attr::Output {
        self.fields(&ctx.contexts_uncons())
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use core::marker::PhantomData;

    use type_fields::cons::ConsRef;

    use crate::prelude::{
        Attribute, Contexts, Field, FieldOperator, FieldsContextsUncons, Operator,
    };

    #[test]
    pub fn test_field_contexts() {
        let context = (1usize, 2.0, "three").cons_ref();

        let (_float, (_int, ())) = Contexts::<_, (&f32, (&usize, ()))>::contexts(context);

        #[derive(Default)]
        pub struct TestFieldsContextsUncons;

        #[derive(Debug, Default, Copy, Clone)]
        pub struct AttrTestFieldsContextsUncons<'a>(PhantomData<&'a ()>);

        impl<'a> Attribute for AttrTestFieldsContextsUncons<'a> {
            type Input = (&'a f32, &'a usize);
            type Output = (usize, f32);
        }

        impl Field<AttrTestFieldsContextsUncons<'_>> for TestFieldsContextsUncons {
            fn field(
                &self,
                (float, int): &<AttrTestFieldsContextsUncons as Attribute>::Input,
            ) -> <AttrTestFieldsContextsUncons as Attribute>::Output {
                (**int, **float)
            }
        }

        #[derive(Default)]
        pub struct TestFieldsContextsUncons2;

        #[derive(Debug, Default, Copy, Clone)]
        pub struct AttrTestFieldsContextsUncons2<'a>(PhantomData<&'a ()>);

        impl<'a> Attribute for AttrTestFieldsContextsUncons2<'a> {
            type Input = (&'a f32, &'a usize);
            type Output = (&'static str, u8);
        }

        impl<Sdf> FieldOperator<Sdf, AttrTestFieldsContextsUncons2<'_>> for TestFieldsContextsUncons2 {
            fn operator(
                &self,
                _: &Sdf,
                (float, _): &<AttrTestFieldsContextsUncons2<'_> as Attribute>::Input,
            ) -> <AttrTestFieldsContextsUncons2<'_> as Attribute>::Output {
                ("hello", **float as u8)
            }
        }

        impl<'a, Sdf> FieldOperator<Sdf, AttrTestFieldsContextsUncons<'a>> for TestFieldsContextsUncons2
        where
            Sdf: Field<AttrTestFieldsContextsUncons<'a>>,
        {
            fn operator(
                &self,
                sdf: &Sdf,
                input: &<AttrTestFieldsContextsUncons<'a> as Attribute>::Input,
            ) -> <AttrTestFieldsContextsUncons<'a> as Attribute>::Output {
                sdf.field(input)
            }
        }

        let ((int, float), ((string, smallint), ())) = FieldsContextsUncons::<
            (
                AttrTestFieldsContextsUncons,
                (AttrTestFieldsContextsUncons2, ()),
            ),
            _,
            _,
        >::fields_contexts_uncons(
            &Operator::<TestFieldsContextsUncons2, TestFieldsContextsUncons>::default(),
            context,
        );

        panic!("{int:} {float:} {string:} {smallint:}");
    }
}
