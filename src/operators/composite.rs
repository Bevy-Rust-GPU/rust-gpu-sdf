//! Operators composed from other operators.

use super::{translate::Translate, rotate::Rotate, scale::Scale, round::Round, hollow::Hollow};

/// Translate, rotate, and scale the wrapped SDF.
pub type Transform<Sdf> = Translate<Rotate<Scale<Sdf>>>;

/// Converts a solid shape into a hollow one with the given surface depth.
pub type Onion<Sdf> = Round<Hollow<Sdf>>;

