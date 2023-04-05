use crate::prelude::{AttributeRef, Context, FieldContext};

/// Function associating an attribute value with a point in space.
///
/// API extension trait of `Field`;
/// moves the `Attr` generic into the function position,
/// and obscures the `attr` parameter using `Attribute`'s `Default` constraint
pub trait FieldAttributeContext<'a, Ctx, State> {
    fn attribute_context<Attr>(&self, p: Ctx) -> Attr::Output
    where
        Self: FieldContext<'a, Attr, Ctx, State>,
        Attr: AttributeRef<'a>,
        Ctx: Context<State, &'a Attr::Input>;
}

impl<'a, T, Ctx, State> FieldAttributeContext<'a, Ctx, State> for T {
    fn attribute_context<Attr>(&self, ctx: Ctx) -> Attr::Output
    where
        T: FieldContext<'a, Attr, Ctx, State>,
        Attr: AttributeRef<'a>,
        Ctx: Context<State, &'a Attr::Input>,
    {
        self.field_context(ctx)
    }
}

