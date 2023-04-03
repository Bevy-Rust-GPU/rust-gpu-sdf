use super::Context;

pub enum Car {}
pub enum Cdr {}
//pub enum This {}

/// Type-level path component for traversing a Context
///
/// impls over `(LHS, RHS)` and `(LHS, ())`
/// to allow traversing arbitrary lengths of list
pub trait ContextPath<'a, Ctx, In> {
    type Type;
}

//impl Sealed for This {}

impl<'a, RHS, Ctx, In> ContextPath<'a, Ctx, In> for (Cdr, RHS)
where
    RHS: ContextPath<'a, Ctx, In>,
    Ctx: Context<RHS, In>,
{
    type Type = In;
}

impl<'a, Ctx, In, RHS> ContextPath<'a, Ctx, In> for (Car, RHS)
where
    Ctx: Context<(Car, RHS), In>,
{
    type Type = In;
}

// The target value is `Self`
//impl<'a, Ctx> ContextPath<Ctx, Ctx> for This where Ctx: Context<'a, This, Ctx> {}
