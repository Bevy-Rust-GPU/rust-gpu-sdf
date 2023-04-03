//! Function associating an attribute value with a point in space.

use crate::prelude::{Attribute, Context, Field};

/// Function associating an attribute value with a point in space.
///
/// API extension trait of `Field`;
/// moves the `Attr` generic into the function position,
/// and obscures the `attr` parameter using `Attribute`'s `Default` constraint
pub trait FieldAttribute {
    fn attribute<Attr>(&self, input: &Attr::Input) -> Attr::Output
    where
        Self: Field<Attr>,
        Attr: Attribute;
}

impl<T> FieldAttribute for T {
    fn attribute<Attr>(&self, input: &Attr::Input) -> Attr::Output
    where
        Self: Field<Attr>,
        Attr: Attribute,
    {
        self.field(input)
    }
}

/// Function associating an attribute value with a point in space.
///
/// API extension trait of `Field`;
/// moves the `Attr` generic into the function position,
/// and obscures the `attr` parameter using `Attribute`'s `Default` constraint
pub trait FieldAttributeContext<Ctx, State> {
    fn attribute_context<Attr>(&self, p: &Ctx) -> Attr::Output
    where
        Self: Field<Attr>,
        Attr: Attribute,
        Ctx: Context<State, Attr::Input>;
}

impl<T, Ctx, State> FieldAttributeContext<Ctx, State> for T {
    fn attribute_context<Attr>(&self, ctx: &Ctx) -> Attr::Output
    where
        T: Field<Attr>,
        Attr: Attribute,
        Ctx: Context<State, Attr::Input>,
    {
        self.field(ctx.context())
    }
}
