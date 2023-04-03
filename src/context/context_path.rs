use super::Context;

pub enum Car {}
pub enum Cdr {}
//pub enum This {}

/// Type-level path component for traversing a Context
///
/// impls over `(LHS, RHS)` and `(LHS, ())`
/// to allow traversing arbitrary lengths of list
pub trait ContextPath<Ctx, In> {
    type Type;
}

//impl Sealed for This {}

impl<RHS, Ctx, In> ContextPath<Ctx, In> for (Cdr, RHS)
where
    RHS: ContextPath<Ctx, In>,
    Ctx: Context<RHS, In>,
{
    type Type = In;
}

impl<Ctx, In, RHS> ContextPath<Ctx, In> for (Car, RHS)
where
    Ctx: Context<(Car, RHS), In>,
{
    type Type = In;
}

// The target value is `Self`
//impl<'a, Ctx> ContextPath<Ctx, Ctx> for This where Ctx: Context<'a, This, Ctx> {}
