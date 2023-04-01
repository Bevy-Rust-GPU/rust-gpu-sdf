pub mod rotate;
pub mod scale;
pub mod translate;

use rust_gpu_bridge::glam::Quat;

use crate::prelude::{Rotate3d, Scale, Translate};

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

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use rust_gpu_bridge::glam::{Quat, Vec3};
    use type_fields::field::Field;

    use crate::{
        prelude::{Cube, Sphere, Transform},
        test_op_attrs_3d,
    };

    #[test]
    fn test_transform() {
        Transform::<Cube, Vec3>::default()
            .with(Transform::rotation, Quat::default())
            .with(Transform::scale, f32::default());
    }

    test_op_attrs_3d!(Transform::<Vec3, Sphere>);
}
