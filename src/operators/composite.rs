//! Operators composed from other operators.

use rust_gpu_bridge::glam::Quat;

use super::{
    hollow::Hollow, isosurface::Isosurface, rotate::Rotate3d, scale::Scale, translate::Translate,
};

/// Translate, rotate, and scale the wrapped SDF.
pub type Transform<Dim, Sdf> = Translate<Dim, Rotate3d<Scale<Sdf>>>;

impl<Dim, Sdf> Transform<Dim, Sdf> {
    pub fn rotation(&mut self) -> &mut Quat {
        &mut self.target.op.rotation
    }

    pub fn scale(&mut self) -> &mut f32 {
        &mut self.target.target.op.scale
    }
}

/// Converts a solid shape into a hollow one with the given surface depth.
pub type Onion<Sdf> = Isosurface<Hollow<Sdf>>;

impl<Sdf> Onion<Sdf> {
    pub fn radius(&mut self) -> &mut f32 {
        &mut self.op.delta
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use rust_gpu_bridge::prelude::Quat;
    use type_fields::field::Field;

    use crate::{
        signed_distance_field::shapes::composite::{Circle, Cube},
        D3,
    };

    use super::{Onion, Transform};

    #[test]
    fn test_transform() {
        Transform::<Cube, D3>::default()
            .with(Transform::rotation, Quat::default())
            .with(Transform::scale, f32::default());
    }

    #[test]
    fn test_onion() {
        Onion::<Circle>::default().with(Onion::radius, f32::default());
    }
}
