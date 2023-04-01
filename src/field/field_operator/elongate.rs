//! Extrude a shape along its axes, preserving exterior geometry.

use core::ops::Add;

use rust_gpu_bridge::{
    glam::{Vec2, Vec3, Vec3Swizzles},
    Abs, Sign,
};
use type_fields::Field;

use crate::prelude::{Distance, Field, FieldOperator, Normal, Operator, Uv};

/// Extrude a shape along its axes, preserving exterior geometry.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
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

impl<Sdf> FieldOperator<Sdf, f32, Distance> for ElongateOp<f32>
where
    Sdf: Field<f32, Distance>,
{
    fn operator(&self, attr: Distance, sdf: &Sdf, p: f32) -> f32 {
        let q = p.abs() - self.extent;
        sdf.field(attr, q.max(0.0)).add(q.min(0.0))
    }
}

impl<Sdf> FieldOperator<Sdf, Vec2, Distance> for ElongateOp<Vec2>
where
    Sdf: Field<Vec2, Distance>,
{
    fn operator(&self, attr: Distance, sdf: &Sdf, p: Vec2) -> f32 {
        let q = p.abs() - self.extent;
        sdf.field(attr, q.max(Vec2::ZERO))
            .add(q.x.max(q.y).min(0.0))
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Distance> for ElongateOp<Vec3>
where
    Sdf: Field<Vec3, Distance>,
{
    fn operator(&self, attr: Distance, sdf: &Sdf, p: Vec3) -> f32 {
        let q = p.abs() - self.extent;
        sdf.field(attr, q.max(Vec3::ZERO))
            .add(q.x.max(q.y.max(q.z)).min(0.0))
    }
}

impl<Sdf> FieldOperator<Sdf, f32, Normal<f32>> for ElongateOp<f32>
where
    Sdf: Field<f32, Normal<f32>>,
{
    fn operator(&self, _attr: Normal<f32>, _sdf: &Sdf, p: f32) -> f32 {
        p.sign()
    }
}

impl<Sdf> FieldOperator<Sdf, Vec2, Normal<Vec2>> for ElongateOp<Vec2>
where
    Sdf: Field<Vec2, Normal<Vec2>>,
{
    fn operator(&self, _attr: Normal<Vec2>, _sdf: &Sdf, p: Vec2) -> Vec2 {
        let w = p.abs() - self.extent;
        let s = p.sign();

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

        m
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Normal<Vec3>> for ElongateOp<Vec3>
where
    Sdf: Field<Vec3, Normal<Vec3>>,
{
    fn operator(&self, _attr: Normal<Vec3>, _sdf: &Sdf, p: Vec3) -> Vec3 {
        let w = p.abs() - self.extent;
        let s = p.sign();

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

        m
    }
}

impl<Sdf> FieldOperator<Sdf, f32, Uv> for ElongateOp<f32>
where
    Sdf: Field<f32, Uv>,
{
    fn operator(&self, _attr: Uv, _sdf: &Sdf, p: f32) -> Vec2 {
        Vec2::new((p + self.extent) * (0.5 / self.extent), 0.0)
    }
}

impl<Sdf> FieldOperator<Sdf, Vec2, Uv> for ElongateOp<Vec2>
where
    Sdf: Field<Vec2, Uv>,
{
    fn operator(&self, _attr: Uv, _sdf: &Sdf, p: Vec2) -> Vec2 {
        (p + self.extent) * (0.5 / self.extent)
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Uv> for ElongateOp<Vec3>
where
    Sdf: Field<Vec3, Uv>,
{
    fn operator(&self, _attr: Uv, _sdf: &Sdf, p: Vec3) -> Vec2 {
        let w = p.abs();

        let m = if w.x > w.y {
            if w.x > w.z {
                (p.zy() + self.extent.zy()) * (0.5 / self.extent.zy())
            } else {
                (p.xy() + self.extent.xy()) * (0.5 / self.extent.xy())
            }
        } else {
            if w.y > w.z {
                (p.xz() + self.extent.xz()) * (0.5 / self.extent.xz())
            } else {
                (p.xy() + self.extent.xy()) * (0.5 / self.extent.xy())
            }
        };

        m
    }
}

#[cfg(feature = "glam")]
use rust_gpu_bridge::{format, Named, String, ToString};

#[cfg(feature = "glam")]
impl<Dim> Named for ElongateOp<Dim>
where
    Dim: Named,
{
    fn module() -> String {
        module_path!().to_string()
    }

    fn short_name() -> String {
        format!("ElongateOp<{}>", Dim::short_name())
    }

    fn name() -> String {
        format!("{}::ElongateOp<{}>", Self::module(), Dim::name())
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
