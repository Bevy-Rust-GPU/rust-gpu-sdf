//! Function associating several attribute values with a point in space.

use crate::prelude::{Attribute, Attributes, Field};

/// Function associating several attribute values with a point in space.
///
/// Extension trait of `Field`;
/// impls over `(LHS, RHS)` and `(LHS, ())` to allow recursive
/// evaluation of `Attribute` cons lists.
pub trait Fields<In, Attr>
where
    Attr: Attributes,
{
    fn fields(&self, attr: Attr, p: In) -> Attr::Type;
}

impl<T, In, LHS, RHS> Fields<In, (LHS, RHS)> for T
where
    T: Field<In, LHS> + Fields<In, RHS>,
    In: Clone,
    LHS: Attribute,
    RHS: Attributes,
{
    fn fields(&self, (lhs, rhs): (LHS, RHS), p: In) -> <(LHS, RHS) as Attributes>::Type {
        (self.field(lhs, p.clone()), self.fields(rhs, p))
    }
}

impl<T, In, LHS> Fields<In, (LHS, ())> for T
where
    T: Field<In, LHS>,
    In: Clone,
    LHS: Attribute,
{
    fn fields(&self, (lhs, _): (LHS, ()), p: In) -> <(LHS, ()) as Attributes>::Type {
        (self.field(lhs, p.clone()), ())
    }
}

