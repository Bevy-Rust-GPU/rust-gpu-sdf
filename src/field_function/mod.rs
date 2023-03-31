pub mod metric;
pub mod shape;

use type_fields::cons::{Cons, Uncons};

use crate::prelude::{Attribute, Attributes};

/// Function associating an attribute value with a point in space.
pub trait FieldFunction<Pos, Attr>
where
    Attr: Attribute,
{
    fn field(&self, attr: Attr, p: Pos) -> Attr::Type;
}

/// Function associating an attribute value with a point in space.
///
/// API extension trait of `FieldAttributeImpl`;
/// moves the `Attr` generic into the function position,
/// and obscures the `attr` parameter using `Attribute`'s `Default` constraint
pub trait FieldAttribute<In> {
    fn attribute<Attr>(&self, p: In) -> Attr::Type
    where
        Self: FieldFunction<In, Attr>,
        Attr: Default + Attribute;
}

impl<T, In> FieldAttribute<In> for T {
    fn attribute<Attr>(&self, p: In) -> Attr::Type
    where
        Self: FieldFunction<In, Attr>,
        Attr: Default + Attribute,
    {
        self.field(Attr::default(), p)
    }
}

/// Function associating several attribute values with a point in space.
///
/// Extension trait of `FieldAttributeImpl`;
/// impls over `(LHS, RHS)` and `(LHS, ())` to allow recursive
/// evaluation of `Attribute` cons lists.
pub trait FieldsFunction<In, Attr>
where
    Attr: Attributes,
{
    fn fields(&self, attr: Attr, p: In) -> Attr::Type;
}

impl<T, In, LHS, RHS> FieldsFunction<In, (LHS, RHS)> for T
where
    T: FieldFunction<In, LHS> + FieldsFunction<In, RHS>,
    In: Clone,
    LHS: Attribute,
    RHS: Attributes,
{
    fn fields(&self, (lhs, rhs): (LHS, RHS), p: In) -> <(LHS, RHS) as Attributes>::Type {
        (self.field(lhs, p.clone()), self.fields(rhs, p))
    }
}

impl<T, In, LHS> FieldsFunction<In, (LHS, ())> for T
where
    T: FieldFunction<In, LHS>,
    In: Clone,
    LHS: Attribute,
{
    fn fields(&self, (lhs, _): (LHS, ()), p: In) -> <(LHS, ()) as Attributes>::Type {
        (self.field(lhs, p.clone()), ())
    }
}

/// Function associating several attribute values with a point in space.
///
/// Extension trait of `FieldAttributesImpl`;
/// `Cons`es the provided tuple, evaluates it via `FieldAttributesImpl`,
/// then returns the `Uncons` of the result.
pub trait FieldAttributes<In, const COUNT: usize> {
    fn attributes<Attr>(
        &self,
        p: In,
    ) -> <<Attr::Cons as Attributes>::Type as Uncons<COUNT>>::Uncons
    where
        Self: FieldsFunction<In, Attr::Cons>,
        Attr: Default + Cons,
        Attr::Cons: Attributes,
        <Attr::Cons as Attributes>::Type: Uncons<COUNT>;
}

impl<T, In, const COUNT: usize> FieldAttributes<In, COUNT> for T {
    fn attributes<Attr>(&self, p: In) -> <<Attr::Cons as Attributes>::Type as Uncons<COUNT>>::Uncons
    where
        Self: FieldsFunction<In, Attr::Cons>,
        Attr: Default + Cons,
        Attr::Cons: Attributes,
        <Attr::Cons as Attributes>::Type: Uncons<COUNT>,
    {
        self.fields(Attr::default().cons(), p).uncons()
    }
}

#[cfg(feature = "glam")]
pub mod boxed {
    extern crate alloc;
    use alloc::boxed::Box;

    use crate::prelude::{Attribute, FieldFunction};

    impl<In, Attr> FieldFunction<In, Attr> for Box<dyn FieldFunction<In, Attr>>
    where
        Attr: Attribute,
    {
        fn attribute(&self, attr: Attr, p: In) -> <Attr as Attribute>::Type {
            self.as_ref().evaluate(attr, p)
        }
    }
}
