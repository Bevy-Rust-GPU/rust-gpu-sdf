//! Function associating an attribute value with a point in space.

use crate::prelude::{Attribute, Context, Field};

/// Function associating an attribute value with a point in space.
///
/// API extension trait of `Field`;
/// moves the `Attr` generic into the function position,
/// and obscures the `attr` parameter using `Attribute`'s `Default` constraint
pub trait FieldAttribute {
    fn attribute<Attr>(&self, p: Attr::Input) -> Attr::Output
    where
        Self: Field<Attr>,
        Attr: Attribute;
}

impl<T> FieldAttribute for T {
    fn attribute<Attr>(&self, p: Attr::Input) -> Attr::Output
    where
        Self: Field<Attr>,
        Attr: Attribute,
    {
        self.field(p)
    }
}

/// Function associating an attribute value with a point in space.
///
/// API extension trait of `Field`;
/// moves the `Attr` generic into the function position,
/// and obscures the `attr` parameter using `Attribute`'s `Default` constraint
pub trait FieldAttributeContext<'a, Ctx, State> {
    fn attribute_context<Attr>(&'a self, p: &'a Ctx) -> Attr::Output
    where
        Self: Field<Attr>,
        Attr: Attribute,
        Attr::Input: Clone + 'a,
        Ctx: Context<'a, State, Attr::Input>;
}

impl<'a, T, Ctx, State> FieldAttributeContext<'a, Ctx, State> for T {
    fn attribute_context<Attr>(&'a self, ctx: &'a Ctx) -> Attr::Output
    where
        T: Field<Attr>,
        Attr: Attribute,
        Attr::Input: Clone + 'a,
        Ctx: Context<'a, State, Attr::Input>,
    {
        self.field(ctx.context().clone())
    }
}

