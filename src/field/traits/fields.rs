//! Function associating several attribute values with a point in space.

use crate::prelude::{Attribute, Attributes, Field};

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
    fn fields(
        &self,
        input: &<(LHS, RHS) as Attributes>::Input,
    ) -> <(LHS, RHS) as Attributes>::Output {
        (self.field(input), self.fields(input))
    }
}

impl<T, LHS> Fields<(LHS, ())> for T
where
    T: Field<LHS>,
    LHS: Attribute,
{
    fn fields(
        &self,
        input: &<(LHS, ()) as Attributes>::Input,
    ) -> <(LHS, ()) as Attributes>::Output {
        (self.field(input), ())
    }
}
