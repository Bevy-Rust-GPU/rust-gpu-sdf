pub enum Car {}
pub enum Cdr {}
pub enum This {}

/// Type-level path component for traversing a Context
///
/// impls over `(LHS, RHS)` and `(LHS, ())`
/// to allow traversing arbitrary lengths of list
pub trait ContextPath {}

impl<LHS, RHS> ContextPath for (LHS, RHS)
where
    LHS: ContextPath,
    RHS: ContextPath,
{
}

impl<LHS> ContextPath for (LHS, ()) where LHS: ContextPath {}

/// The target value lies in the left hand side of the current cell
impl ContextPath for Car {}

/// The target value lies in the right hand side of the current cell
impl ContextPath for Cdr {}

/// The target value is `Self`
impl ContextPath for This {}

