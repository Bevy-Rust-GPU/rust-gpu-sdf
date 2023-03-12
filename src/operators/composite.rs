//! Operators composed from other operators.

use super::{
    hollow::Hollow, isosurface::Isosurface, rotate::Rotate3d, scale::Scale, translate::Translate,
    Operator,
};

/// Translate, rotate, and scale the wrapped SDF.
pub type Transform<Sdf, Dim> = Translate<Rotate3d<Scale<Sdf>>, Dim>;

#[allow(non_camel_case_types)]
pub type Transform_Rotation = (
    crate::operators::Operator_Target,
    crate::operators::rotate::Rotate3d_Rotation,
);

#[allow(non_camel_case_types)]
pub type Transform_Scale = (
    crate::operators::Operator_Target,
    crate::operators::Operator_Target,
    crate::operators::scale::Scale_Scale,
);

impl<Sdf, Dim> Transform<Sdf, Dim> {
    pub const ROTATION: Transform_Rotation = (Operator::<(), ()>::TARGET, Rotate3d::<()>::ROTATION);

    pub const SCALE: Transform_Scale = (
        Operator::<(), ()>::TARGET,
        Operator::<(), ()>::TARGET,
        Scale::<()>::SCALE,
    );
}

/// Converts a solid shape into a hollow one with the given surface depth.
pub type Onion<Sdf> = Isosurface<Hollow<Sdf>>;

#[allow(non_camel_case_types)]
pub type Onion_Radius = crate::operators::isosurface::Isosurface_Delta;

impl<Sdf> Onion<Sdf> {
    pub const RADIUS: Onion_Radius = Isosurface::<()>::DELTA;
}

#[cfg(test)]
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
            .with(Transform::<(), ()>::ROTATION, Quat::default())
            .with(Transform::<(), ()>::SCALE, f32::default());
    }

    #[test]
    fn test_onion() {
        Onion::<Circle>::default().with(Onion::<()>::RADIUS, f32::default());
    }
}
