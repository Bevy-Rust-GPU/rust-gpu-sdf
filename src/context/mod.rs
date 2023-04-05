//! Type-level data access

pub mod traits;
pub mod items;

/// Marker denoting the left side of a cons cell
pub enum Car {}

/// Marker denoting the right side of a cons cell
pub enum Cdr {}

/// Marker denoting self
pub enum This {}
