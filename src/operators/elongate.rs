//! Extrude a shape along its axes, preserving exterior geometry.

use core::ops::Add;

use rust_gpu_bridge::{
    glam::{Vec2, Vec3, Vec3Swizzles},
    Abs, Sign,
};
use type_fields::Field;

use crate::{
    prelude::{Distance, DistanceFunction, Operator, SignedDistanceOperator},
    signed_distance_field::attributes::{normal::Normal, uv::Uv},
};

/// Extrude a shape along its axes, preserving exterior geometry.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct ElongateOp<Dim> {
    pub extent: Dim,
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

impl<Sdf> SignedDistanceOperator<Sdf, f32, Distance> for ElongateOp<f32>
where
    Sdf: DistanceFunction<f32, Distance>,
{
    fn operator(&self, attr: Distance, sdf: &Sdf, p: f32) -> f32 {
        let q = p.abs() - self.extent;
        sdf.evaluate(attr, q.max(0.0)).add(q.min(0.0))
    }
}

impl<Sdf> SignedDistanceOperator<Sdf, Vec2, Distance> for ElongateOp<Vec2>
where
    Sdf: DistanceFunction<Vec2, Distance>,
{
    fn operator(&self, attr: Distance, sdf: &Sdf, p: Vec2) -> f32 {
        let q = p.abs() - self.extent;
        sdf.evaluate(attr, q.max(Vec2::ZERO))
            .add(q.x.max(q.y).min(0.0))
    }
}

impl<Sdf> SignedDistanceOperator<Sdf, Vec3, Distance> for ElongateOp<Vec3>
where
    Sdf: DistanceFunction<Vec3, Distance>,
{
    fn operator(&self, attr: Distance, sdf: &Sdf, p: Vec3) -> f32 {
        let q = p.abs() - self.extent;
        sdf.evaluate(attr, q.max(Vec3::ZERO))
            .add(q.x.max(q.y.max(q.z)).min(0.0))
    }
}

impl<Sdf> SignedDistanceOperator<Sdf, f32, Normal<f32>> for ElongateOp<f32>
where
    Sdf: DistanceFunction<f32, Normal<f32>>,
{
    fn operator(&self, attr: Normal<f32>, _sdf: &Sdf, p: f32) -> f32 {
        p.sign()
    }
}

impl<Sdf> SignedDistanceOperator<Sdf, Vec2, Normal<Vec2>> for ElongateOp<Vec2>
where
    Sdf: DistanceFunction<Vec2, Normal<Vec2>>,
{
    fn operator(&self, attr: Normal<Vec2>, _sdf: &Sdf, p: Vec2) -> Vec2 {
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

impl<Sdf> SignedDistanceOperator<Sdf, Vec3, Normal<Vec3>> for ElongateOp<Vec3>
where
    Sdf: DistanceFunction<Vec3, Normal<Vec3>>,
{
    fn operator(&self, attr: Normal<Vec3>, _sdf: &Sdf, p: Vec3) -> Vec3 {
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

impl<Sdf> SignedDistanceOperator<Sdf, Vec2, Uv> for ElongateOp<Vec2>
where
    Sdf: DistanceFunction<Vec2, Uv>,
{
    fn operator(&self, attr: Uv, _sdf: &Sdf, p: Vec2) -> Vec2 {
        (p + self.extent) * (0.5 / self.extent)
    }
}

impl<Sdf> SignedDistanceOperator<Sdf, Vec3, Uv> for ElongateOp<Vec3>
where
    Sdf: DistanceFunction<Vec3, Uv>,
{
    fn operator(&self, attr: Uv, _sdf: &Sdf, p: Vec3) -> Vec2 {
        let w = p.abs();

        let m = if w.x > w.y {
            if w.x > w.z {
                (p.zy() + self.extent.zy()) * (0.5 / self.extent.zy())
            } else {
                (p.xy() + self.extent.xy()) * (0.5 / self.extent.xy())
            }
        } else {
            if w.y > w.z {
                (p.zy() + self.extent.zy()) * (0.5 / self.extent.zy())
            } else {
                (p.xy() + self.extent.xy()) * (0.5 / self.extent.xy())
            }
        };

        m
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
    use rust_gpu_bridge::glam::Vec3;
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Point;

    use super::Elongate;

    #[test]
    fn test_elongate() {
        Elongate::<_, Point>::default().with(Elongate::extent, Vec3::default());
    }
}
