use type_fields::cons::Uncons;

use crate::prelude::{ConsAttributes, Context, FieldsContext};

/// Function associating several attribute values with a point in space.
///
/// Extension trait of `FieldsContext`;
/// `Cons`es the provided tuple, evaluates it via `FieldAttributesImpl`,
/// then returns the `Uncons` of the result.
pub trait FieldAttributesContext<'a, Ctx, State> {
    fn attributes_context<Attr>(&self, ctx: Ctx) -> Attr::UnconsOutput
    where
        Self: FieldsContext<'a, Ctx, State, Attr::Cons>,
        Ctx: Context<State, &'a Attr::AttrInput>,
        Attr: ConsAttributes<'a>;
}

impl<'a, T, Ctx, State> FieldAttributesContext<'a, Ctx, State> for T {
    fn attributes_context<Attr>(&self, ctx: Ctx) -> Attr::UnconsOutput
    where
        Self: FieldsContext<'a, Ctx, State, Attr::Cons>,
        Ctx: Context<State, &'a Attr::AttrInput>,
        Attr: ConsAttributes<'a>,
    {
        self.fields_context(ctx).uncons()
    }
}

