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
    fn fields(&self, p: Attr::Input) -> Attr::Output;
}

impl<T, LHS, RHS> Fields<(LHS, RHS)> for T
where
    T: Field<LHS> + Fields<RHS>,
    LHS: Attribute,
    LHS::Input: Clone,
    RHS: Attributes<Input = LHS::Input>,
{
    fn fields(&self, p: <(LHS, RHS) as Attributes>::Input) -> <(LHS, RHS) as Attributes>::Output {
        (self.field(p.clone()), self.fields(p))
    }
}

impl<T, LHS> Fields<(LHS, ())> for T
where
    T: Field<LHS>,
    LHS: Attribute,
    LHS::Input: Clone,
{
    fn fields(&self, p: <(LHS, ()) as Attributes>::Input) -> <(LHS, ()) as Attributes>::Output {
        (self.field(p), ())
    }
}

/// Function associating an attribute value with a point in space.
///
/// API extension trait of `Field`;
/// moves the `Attr` generic into the function position,
/// and obscures the `attr` parameter using `Attribute`'s `Default` constraint
pub trait FieldsContext<'a, Ctx, State> {
    fn fields_context<Attr>(&'a self, p: &'a Ctx) -> Attr::Output
    where
        Self: Fields<Attr>,
        Attr: Attributes,
        Attr::Input: Clone + 'a,
        Ctx: Context<'a, State, Attr::Input>;
}

impl<'a, T, Ctx, State> FieldsContext<'a, Ctx, State> for T {
    fn fields_context<Attr>(&'a self, ctx: &'a Ctx) -> Attr::Output
    where
        Self: Fields<Attr>,
        Attr: Attributes,
        Attr::Input: Clone + 'a,
        Ctx: Context<'a, State, Attr::Input>,
    {
        self.fields(ctx.context().clone())
    }
}
