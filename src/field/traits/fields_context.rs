use crate::prelude::{AttributesRef, Context, Fields};

/// Function associating an attribute value with a point in space.
///
/// API extension trait of `Fields`;
/// moves the `Attr` generic into the function position,
/// and obscures the `attr` parameter using `Attribute`'s `Default` constraint
pub trait FieldsContext<'a, Ctx, State, Attr>: Fields<Attr>
where
    Attr: AttributesRef<'a>,
    Ctx: Context<State, &'a Attr::Input>,
{
    fn fields_context(&self, p: Ctx) -> Attr::Output;
}

impl<'a, T, Ctx, State, Attr> FieldsContext<'a, Ctx, State, Attr> for T
where
    Self: Fields<Attr>,
    Attr: AttributesRef<'a>,
    Ctx: Context<State, &'a Attr::Input>,
{
    fn fields_context(&self, ctx: Ctx) -> Attr::Output {
        self.fields(ctx.context())
    }
}

