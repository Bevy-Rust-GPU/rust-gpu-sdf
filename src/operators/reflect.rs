//! Reflect a distance field about an arbitrary axis.

use rust_gpu_bridge::{
    prelude::{Vec2, Vec3},
    reflect::Reflect as ReflectTrait,
};
use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

/// Reflect a distance field about an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
pub struct ReflectOp<Dim> {
    pub axis: Dim,
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

impl SignedDistanceOperator<Vec2, Distance> for ReflectOp<Vec2> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> Distance
    where
        Sdf: SignedDistanceField<Vec2, Distance>,
    {
        assert!(
            self.axis.is_normalized(),
            "ReflectOp axis must be normalized"
        );
        let q = if p.dot(self.axis) >= 0.0 {
            p
        } else {
            p.reflect(self.axis)
        };
        sdf.evaluate(q)
    }
}

impl SignedDistanceOperator<Vec3, Distance> for ReflectOp<Vec3> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> Distance
    where
        Sdf: SignedDistanceField<Vec3, Distance>,
    {
        assert!(
            self.axis.is_normalized(),
            "ReflectOp axis must be normalized"
        );
        let q = if p.dot(self.axis) >= 0.0 {
            p
        } else {
            p.reflect(self.axis)
        };
        sdf.evaluate(q)
    }
}

/// Reflect a distance field about an arbitrary axis.
pub type Reflect<Sdf, Dim> = Operator<Sdf, ReflectOp<Dim>>;

impl<Sdf, Dim> Reflect<Sdf, Dim> {
    pub fn axis(&mut self) -> &mut Dim {
        &mut self.op.axis
    }
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::Vec3;
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Sphere;

    use super::Reflect;

    #[test]
    fn test_reflect() {
        Reflect::<Sphere, _>::default().with(Reflect::axis, Vec3::default());
    }
}
