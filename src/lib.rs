#![no_std]

pub mod markers;
pub mod signed_distance_field;
pub mod operators;

#[cfg(test)]
pub mod tests;

/// Free-standing Default::default() invocation
pub fn default<T: Default>() -> T {
    Default::default()
}
