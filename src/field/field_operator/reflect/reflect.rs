//! Reflect a distance field about an arbitrary axis.

use core::ops::{Mul, Sub};

use rust_gpu_bridge::{
    glam::{Vec2, Vec3, Vec3Swizzles},
    Dot, IsNormalized, Mix, Reflect as ReflectTrait, Splat, Step,
};
use type_fields::macros::Field;

use crate::prelude::{
    items::position::Position, AttrDistance, AttrNormal, AttrUv, Distance, Field, FieldOperator,
    Normal, Operator, Uv,
};

/// Reflect a distance field about an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
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

impl<Sdf, Input> FieldOperator<Sdf, AttrDistance<Input>> for ReflectOp<Input>
where
    Sdf: Field<AttrDistance<Input>>,
    Input: Clone + Sub<Input, Output = Input> + Mul<f32, Output = Input> + IsNormalized + Dot,
{
    fn operator(&self, sdf: &Sdf, input: &Position<Input>) -> Distance {
        assert!(
            self.axis.clone().is_normalized(),
            "ReflectOp axis must be normalized"
        );

        let q = input.clone()
            - self.axis.clone() * (**input).clone().dot(self.axis.clone()).min(0.0) * 2.0;

        sdf.field(&q)
    }
}

impl<Sdf, Input> FieldOperator<Sdf, AttrNormal<Input>> for ReflectOp<Input>
where
    Sdf: Field<AttrNormal<Input>>,
    Input: Clone
        + Sub<Input, Output = Input>
        + Mul<f32, Output = Input>
        + IsNormalized
        + Dot
        + ReflectTrait
        + Mix
        + Splat,
{
    fn operator(&self, sdf: &Sdf, input: &Position<Input>) -> Normal<Input> {
        assert!(
            self.axis.clone().is_normalized(),
            "ReflectOp axis must be normalized"
        );

        let c = (**input).clone().dot(self.axis.clone());
        let pc = self.axis.clone() * c.min(0.0) * 2.0;
        let q = (**input).clone() - pc;

        let n = (*sdf.field(&q.into())).clone();
        n.clone()
            .mix(n.reflect(self.axis.clone()), Input::splat(c.step(0.0)))
            .into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrUv<f32>> for ReflectOp<f32>
where
    Sdf: Field<AttrUv<f32>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<f32>) -> Uv {
        assert!(
            self.axis.is_normalized(),
            "ReflectOp axis must be normalized"
        );

        let c = p;
        let pc = c.min(0.0) * 2.0;
        let q = **p - pc;

        let n = *sdf.field(&q.into());
        n.mix(n * Vec2::new(-1.0, 1.0), Vec2::splat(c.step(0.0)))
            .into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrUv<Vec2>> for ReflectOp<Vec2>
where
    Sdf: Field<AttrUv<Vec2>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec2>) -> Uv {
        assert!(
            self.axis.is_normalized(),
            "ReflectOp axis must be normalized"
        );

        let c = p.dot(self.axis);
        let pc = self.axis * c.min(0.0) * 2.0;
        let q = **p - pc;

        let n = sdf.field(&q.into());
        n.mix(n.reflect(self.axis), Vec2::splat(c.step(0.0))).into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrUv<Vec3>> for ReflectOp<Vec3>
where
    Sdf: Field<AttrUv<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec3>) -> Uv {
        assert!(
            self.axis.is_normalized(),
            "ReflectOp axis must be normalized"
        );

        let c = p.dot(self.axis);
        let pc = self.axis * c.min(0.0) * 2.0;
        let q = **p - pc;

        let n = sdf.field(&q.into());
        n.mix(n.reflect(self.axis.xy()), Vec2::splat(c.step(0.0)))
            .into()
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
