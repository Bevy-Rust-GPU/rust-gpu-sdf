//! Reflect a distance field about an arbitrary axis.

use core::ops::{Mul, Sub};

use rust_gpu_bridge::{
    glam::{Vec2, Vec3, Vec3Swizzles},
    Dot, Length, Mix, Reflect as ReflectTrait, Splat, Step,
};
use type_fields::Field;

use crate::prelude::{Attribute, Distance, FieldFunction, FieldOperator, Normal, Operator, Uv};

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

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Distance> for ReflectOp<Dim>
where
    Sdf: FieldFunction<Dim, Distance>,
    Dim: Clone + Sub<Dim, Output = Dim> + Mul<f32, Output = Dim> + Length + Dot,
{
    fn operator(&self, attr: Distance, sdf: &Sdf, p: Dim) -> f32 {
        assert!(
            self.axis.clone().length() == 1.0,
            "ReflectOp axis must be normalized"
        );

        let q = p.clone() - self.axis.clone() * p.clone().dot(self.axis.clone()).min(0.0) * 2.0;

        sdf.evaluate(attr, q)
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Normal<Dim>> for ReflectOp<Dim>
where
    Sdf: FieldFunction<Dim, Normal<Dim>>,
    Dim: Clone
        + Sub<Dim, Output = Dim>
        + Mul<f32, Output = Dim>
        + Length
        + Dot
        + ReflectTrait
        + Mix
        + Splat,
{
    fn operator(&self, attr: Normal<Dim>, sdf: &Sdf, p: Dim) -> Dim {
        assert!(
            self.axis.clone().length() == 1.0,
            "ReflectOp axis must be normalized"
        );

        let c = p.clone().dot(self.axis.clone());
        let pc = self.axis.clone() * c.min(0.0) * 2.0;
        let q = p.clone() - pc;

        let n = sdf.evaluate(attr, q);
        n.clone()
            .mix(n.reflect(self.axis.clone()), Dim::splat(c.step(0.0)))
    }
}

impl<Sdf> FieldOperator<Sdf, f32, Uv> for ReflectOp<f32>
where
    Sdf: FieldFunction<f32, Uv>,
{
    fn operator(&self, attr: Uv, sdf: &Sdf, p: f32) -> Vec2 {
        assert!(
            self.axis.length() == 1.0,
            "ReflectOp axis must be normalized"
        );

        let c = p;
        let pc = c.min(0.0) * 2.0;
        let q = p - pc;

        let n = sdf.evaluate(attr, q);
        n.mix(n * Vec2::new(-1.0, 1.0), Vec2::splat(c.step(0.0)))
    }
}

impl<Sdf> FieldOperator<Sdf, Vec2, Uv> for ReflectOp<Vec2>
where
    Sdf: FieldFunction<Vec2, Uv>,
{
    fn operator(&self, attr: Uv, sdf: &Sdf, p: Vec2) -> Vec2 {
        assert!(
            self.axis.length() == 1.0,
            "ReflectOp axis must be normalized"
        );

        let c = p.dot(self.axis);
        let pc = self.axis * c.min(0.0) * 2.0;
        let q = p - pc;

        let n = sdf.evaluate(attr, q);
        n.mix(n.reflect(self.axis), Vec2::splat(c.step(0.0)))
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Uv> for ReflectOp<Vec3>
where
    Sdf: FieldFunction<Vec3, Uv>,
{
    fn operator(&self, attr: Uv, sdf: &Sdf, p: Vec3) -> Vec2 {
        assert!(
            self.axis.length() == 1.0,
            "ReflectOp axis must be normalized"
        );

        let c = p.dot(self.axis);
        let pc = self.axis * c.min(0.0) * 2.0;
        let q = p - pc;

        let n = sdf.evaluate(attr, q);
        n.mix(n.reflect(self.axis.xy()), Vec2::splat(c.step(0.0)))
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
    use rust_gpu_bridge::glam::{Vec2, Vec3};
    use type_fields::field::Field;

    use crate::{
        prelude::{Point, Sphere},
        test_op_attrs_1d, test_op_attrs_2d, test_op_attrs_3d,
    };

    use super::Reflect;

    #[test]
    fn test_reflect() {
        Reflect::<_, Sphere>::default().with(Reflect::axis, Vec3::default());
    }

    test_op_attrs_1d!(Reflect::<f32, Point>);
    test_op_attrs_2d!(Reflect::<Vec2, Point>);
    test_op_attrs_3d!(Reflect::<Vec3, Point>);
}
