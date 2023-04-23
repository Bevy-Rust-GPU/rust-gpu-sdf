use type_fields::t_funk::{hlist::ToTList, tlist::ToHList};

use crate::prelude::{Attributes, AttributesRef, Fields, RegistersUncons};

/// Evalute multiple attributes of a field function,
/// drawing input from `Ctx` and `Uncons`ing the result.
pub trait FieldsRegistersUncons<'a, Attrs, Ctx, State>: Fields<Attrs::HList>
where
    Attrs: ToHList,
    Attrs::HList: AttributesRef<'a>,
    <Attrs::HList as AttributesRef<'a>>::InputRef: ToHList,
    <<Attrs::HList as AttributesRef<'a>>::InputRef as ToHList>::HList:
        ToTList<TList = <Attrs::HList as AttributesRef<'a>>::InputRef>,
    <Attrs::HList as Attributes>::Output: ToTList,
    Ctx: RegistersUncons<
        State,
        <Attrs::HList as AttributesRef<'a>>::InputRef,
        Type = <<Attrs::HList as AttributesRef<'a>>::InputRef as ToHList>::HList,
    >,
{
    fn fields_registers_uncons(&self, ctx: Ctx) -> <Attrs::HList as Attributes>::Output;
}

impl<'a, T, Attrs, Ctx, State> FieldsRegistersUncons<'a, Attrs, Ctx, State> for T
where
    Self: Fields<Attrs::HList>,
    Attrs: ToHList,
    Attrs::HList: AttributesRef<'a>,
    <Attrs::HList as AttributesRef<'a>>::InputRef: ToHList,
    <<Attrs::HList as AttributesRef<'a>>::InputRef as ToHList>::HList:
        ToTList<TList = <Attrs::HList as AttributesRef<'a>>::InputRef>,
    <Attrs::HList as Attributes>::Output: ToTList,
    Ctx: RegistersUncons<
        State,
        <Attrs::HList as AttributesRef<'a>>::InputRef,
        Type = <<Attrs::HList as AttributesRef<'a>>::InputRef as ToHList>::HList,
    >,
{
    fn fields_registers_uncons(&self, ctx: Ctx) -> <Attrs::HList as Attributes>::Output {
        self.fields(&ctx.registers_uncons())
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
mod test {
    use core::marker::PhantomData;

    use type_fields::cons::ConsRef;

    use crate::prelude::{Attribute, Field, FieldOperator, FieldsRegistersUncons, Operator};

    #[test]
    pub fn test_fields_contexts_uncons() {
        let context = (1usize, 2.0, "three").cons_ref();

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

        let ((_int, _float), ((_string, _smallint), ())) = FieldsRegistersUncons::<
            (AttrTestFieldsContextsUncons, AttrTestFieldsContextsUncons2),
            _,
            _,
        >::fields_registers_uncons(
            &Operator::<TestFieldsContextsUncons2, TestFieldsContextsUncons>::default(),
            context,
        );
    }
}
