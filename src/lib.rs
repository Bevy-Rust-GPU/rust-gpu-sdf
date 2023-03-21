#![no_std]

pub mod operators;
pub mod raymarch;
pub mod signed_distance_field;
pub mod bound_checker;
pub use type_fields;

pub mod prelude;

use rust_gpu_bridge::glam::{Vec2, Vec3};

pub type D1 = f32;
pub type D2 = Vec2;
pub type D3 = Vec3;

/// Free-standing [`Default::default()`] invocation
pub fn default<T: Default>() -> T {
    Default::default()
}
