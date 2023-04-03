//! Function associating several attribute values with a point in space.

use crate::prelude::{Attribute, Attributes, Field, Context};

/// Function associating several attribute values with a point in space.
///
/// Extension trait of `Field`;
/// impls over `(LHS, RHS)` and `(LHS, ())` to allow recursive
/// evaluation of `Attribute` cons lists.
pub trait Fields<Attr>
where
    Attr: Attributes,
{
    fn fields(&self, input: &Attr::Input) -> Attr::Output;
}

impl<T, LHS, RHS> Fields<(LHS, RHS)> for T
where
    T: Field<LHS> + Fields<RHS>,
    LHS: Attribute,
    RHS: Attributes<Input = LHS::Input>,
{
    fn fields(&self, input: &<(LHS, RHS) as Attributes>::Input) -> <(LHS, RHS) as Attributes>::Output {
        (self.field(input), self.fields(input))
    }
}

impl<T, LHS> Fields<(LHS, ())> for T
where
    T: Field<LHS>,
    LHS: Attribute,
{
    fn fields(&self, input: &<(LHS, ()) as Attributes>::Input) -> <(LHS, ()) as Attributes>::Output {
        (self.field(input), ())
    }
}

/// Function associating an attribute value with a point in space.
///
/// API extension trait of `Field`;
/// moves the `Attr` generic into the function position,
/// and obscures the `attr` parameter using `Attribute`'s `Default` constraint
pub trait FieldsContext<Ctx, State> {
    fn fields_context<Attr>(&self, p: &Ctx) -> Attr::Output
    where
        Self: Fields<Attr>,
        Attr: Attributes,
        Ctx: Context<State, Attr::Input>;
}

impl<T, Ctx, State> FieldsContext<Ctx, State> for T {
    fn fields_context<Attr>(&self, ctx: &Ctx) -> Attr::Output
    where
        Self: Fields<Attr>,
        Attr: Attributes,
        Ctx: Context<State, Attr::Input>,
    {
        self.fields(ctx.context())
    }
}
