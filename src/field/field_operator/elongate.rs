//! Extrude a shape along its axes, preserving exterior geometry.

use core::ops::Add;

use rust_gpu_bridge::{
    glam::{Vec2, Vec3, Vec3Swizzles},
    Abs, Sign,
};
use type_fields::Field;

use crate::prelude::{
    items::position::Position, AttrDistance, AttrNormal, AttrUv, Distance, Field, FieldOperator,
    Normal, Operator, Uv,
};

/// Extrude a shape along its axes, preserving exterior geometry.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct ElongateOp<Dim> {
    pub extent: Dim,
}

#[cfg(feature = "bevy")]
impl<Dim> bevy::reflect::TypeUuid for ElongateOp<Dim> {
    const TYPE_UUID: bevy::reflect::Uuid = bevy::reflect::Uuid::from_u128(196665527114209003);
}

impl Default for ElongateOp<f32> {
    fn default() -> Self {
        ElongateOp { extent: 1.0 }
    }
}

impl Default for ElongateOp<Vec2> {
    fn default() -> Self {
        ElongateOp { extent: Vec2::ONE }
    }
}

impl Default for ElongateOp<Vec3> {
    fn default() -> Self {
        ElongateOp { extent: Vec3::ONE }
    }
}

impl<Sdf> FieldOperator<Sdf, AttrDistance<f32>> for ElongateOp<f32>
where
    Sdf: Field<AttrDistance<f32>>,
{
    fn operator(&self, sdf: &Sdf, input: &Position<f32>) -> Distance {
        let q = input.abs() - self.extent;
        sdf.field(&q.max(0.0).into()) + q.min(0.0)
    }
}

impl<Sdf> FieldOperator<Sdf, AttrDistance<Vec2>> for ElongateOp<Vec2>
where
    Sdf: Field<AttrDistance<Vec2>>,
{
    fn operator(&self, sdf: &Sdf, input: &Position<Vec2>) -> Distance {
        let q = input.abs() - self.extent;
        sdf.field(&q.max(Vec2::ZERO).into()) + q.x.max(q.y).min(0.0)
    }
}

impl<Sdf> FieldOperator<Sdf, AttrDistance<Vec3>> for ElongateOp<Vec3>
where
    Sdf: Field<AttrDistance<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, input: &Position<Vec3>) -> Distance {
        let q = input.abs() - self.extent;
        sdf.field(&q.max(Vec3::ZERO).into()) + q.x.max(q.y.max(q.z)).min(0.0)
    }
}

impl<Sdf> FieldOperator<Sdf, AttrNormal<f32>> for ElongateOp<f32>
where
    Sdf: Field<AttrNormal<f32>>,
{
    fn operator(&self, _sdf: &Sdf, input: &Position<f32>) -> Normal<f32> {
        input.sign().into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrNormal<Vec2>> for ElongateOp<Vec2>
where
    Sdf: Field<AttrNormal<Vec2>>,
{
    fn operator(&self, _sdf: &Sdf, input: &Position<Vec2>) -> Normal<Vec2> {
        let w = input.abs() - self.extent;
        let s = input.sign();

        let g = w.x.max(w.y);
        let q = w.max(Vec2::ZERO);
        let l = q.length();

        let m = s
            * (if g > 0.0 {
                q / l
            } else {
                if w.x > w.y {
                    Vec2::X
                } else {
                    Vec2::Y
                }
            });

        m.into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrNormal<Vec3>> for ElongateOp<Vec3>
where
    Sdf: Field<AttrNormal<Vec3>>,
{
    fn operator(&self, _sdf: &Sdf, input: &Position<Vec3>) -> Normal<Vec3> {
        let w = input.abs() - self.extent;
        let s = input.sign();

        let g = w.x.max(w.y).max(w.z);
        let q = w.max(Vec3::ZERO);
        let l = q.length();

        let m = s
            * (if g > 0.0 {
                q / l
            } else {
                if w.x > w.y {
                    if w.x > w.z {
                        Vec3::X
                    } else {
                        Vec3::Z
                    }
                } else {
                    if w.y > w.z {
                        Vec3::Y
                    } else {
                        Vec3::Z
                    }
                }
            });

        m.into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrUv<f32>> for ElongateOp<f32>
where
    Sdf: Field<AttrUv<f32>>,
{
    fn operator(&self, _sdf: &Sdf, input: &Position<f32>) -> Uv {
        Vec2::new((**input + self.extent) * (0.5 / self.extent), 0.0).into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrUv<Vec2>> for ElongateOp<Vec2>
where
    Sdf: Field<AttrUv<Vec2>>,
{
    fn operator(&self, _sdf: &Sdf, input: &Position<Vec2>) -> Uv {
        ((**input + self.extent) * (0.5 / self.extent)).into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrUv<Vec3>> for ElongateOp<Vec3>
where
    Sdf: Field<AttrUv<Vec3>>,
{
    fn operator(&self, _sdf: &Sdf, input: &Position<Vec3>) -> Uv {
        let w = input.abs();

        let m = if w.x > w.y {
            if w.x > w.z {
                (input.zy() + self.extent.zy()) * (0.5 / self.extent.zy())
            } else {
                (input.xy() + self.extent.xy()) * (0.5 / self.extent.xy())
            }
        } else {
            if w.y > w.z {
                (input.xz() + self.extent.xz()) * (0.5 / self.extent.xz())
            } else {
                (input.xy() + self.extent.xy()) * (0.5 / self.extent.xy())
            }
        };

        m.into()
    }
}

/// Extrude a shape along its axes, preserving exterior geometry.
pub type Elongate<Dim, Sdf> = Operator<ElongateOp<Dim>, Sdf>;

impl<Dim, Sdf> Elongate<Dim, Sdf> {
    pub fn extent(&mut self) -> &mut Dim {
        &mut self.op.extent
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::glam::{Vec2, Vec3};
    use type_fields::field::Field;

    use crate::{
        prelude::{Elongate, Point},
        test_op_attrs_1d, test_op_attrs_2d, test_op_attrs_3d,
    };

    #[test]
    fn test_elongate() {
        Elongate::<Vec3, Point>::default().with(Elongate::extent, Vec3::default());
    }

    test_op_attrs_1d!(Elongate::<f32, Point>);
    test_op_attrs_2d!(Elongate::<Vec2, Point>);
    test_op_attrs_3d!(Elongate::<Vec3, Point>);
}
