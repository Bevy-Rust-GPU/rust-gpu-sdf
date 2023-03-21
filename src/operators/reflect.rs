//! Reflect a distance field about an arbitrary axis.

use core::ops::{Mul, Sub};

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Dot, Length,
};
use type_fields::Field;

use crate::prelude::{DistanceFunction, Operator, SignedDistanceOperator};

/// Reflect a distance field about an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct ReflectOp<Dim> {
    pub axis: Dim,
}

impl Default for ReflectOp<f32> {
    fn default() -> Self {
        ReflectOp { axis: 1.0 }
    }
}

impl Default for ReflectOp<Vec2> {
    fn default() -> Self {
        ReflectOp { axis: Vec2::X }
    }
}

impl Default for ReflectOp<Vec3> {
    fn default() -> Self {
        ReflectOp { axis: Vec3::X }
    }
}

impl<Sdf, Dim, Out> SignedDistanceOperator<Sdf, Dim, Out> for ReflectOp<Dim>
where
    Sdf: DistanceFunction<Dim, Out>,
    Dim: Clone + Sub<Dim, Output = Dim> + Mul<f32, Output = Dim> + Length + Dot,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Out {
        assert!(
            self.axis.clone().length() == 1.0,
            "ReflectOp axis must be normalized"
        );

        let q = p.clone() - self.axis.clone() * p.clone().dot(self.axis.clone()).min(0.0) * 2.0;

        sdf.evaluate(q)
    }
}

/// Reflect a distance field about an arbitrary axis.
pub type Reflect<Dim, Sdf> = Operator<ReflectOp<Dim>, Sdf>;

impl<Dim, Sdf> Reflect<Dim, Sdf> {
    pub fn axis(&mut self) -> &mut Dim {
        &mut self.op.axis
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::prelude::Vec3;
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Sphere;

    use super::Reflect;

    #[test]
    fn test_reflect() {
        Reflect::<_, Sphere>::default().with(Reflect::axis, Vec3::default());
    }
}
