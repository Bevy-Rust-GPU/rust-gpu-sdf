#![no_std]

pub mod bound_tester;
pub mod field_operator;
pub use type_fields;
pub mod attribute;
pub mod field_function;

pub mod prelude;

use rust_gpu_bridge::glam::{Vec2, Vec3};

pub type D1 = f32;
pub type D2 = Vec2;
pub type D3 = Vec3;

/// Free-standing [`Default::default()`] invocation
pub fn default<T: Default>() -> T {
    Default::default()
}
