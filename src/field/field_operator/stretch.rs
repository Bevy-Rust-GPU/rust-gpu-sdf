//! Stretch a shape along an arbitrary axis, preserving exterior geometry as caps.

use core::ops::{Mul, Sub};

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Dot, IsNormalized,
};
use type_fields::Field;

use crate::prelude::{Attribute, Field, FieldOperator, Operator};

/// Extrude a shape infinitely along an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct StretchInfiniteOp<Dim> {
    pub dir: Dim,
}

impl Default for StretchInfiniteOp<f32> {
    fn default() -> Self {
        StretchInfiniteOp { dir: 1.0 }
    }
}

impl Default for StretchInfiniteOp<Vec2> {
    fn default() -> Self {
        StretchInfiniteOp { dir: Vec2::X }
    }
}

impl Default for StretchInfiniteOp<Vec3> {
    fn default() -> Self {
        StretchInfiniteOp { dir: Vec3::X }
    }
}

impl<Sdf, Dim, Attr> FieldOperator<Sdf, Attr> for StretchInfiniteOp<Dim>
where
    Attr: Attribute<Input = Dim>,
    Sdf: Field<Attr>,
    Dim: Clone + Mul<f32, Output = Dim> + Sub<Dim, Output = Dim> + IsNormalized + Dot,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Attr::Output {
        assert!(
            self.dir.clone().is_normalized(),
            "ExtrudeInfiniteOp dir must be normalized"
        );
        let q = p.clone() - self.dir.clone() * p.dot(self.dir.clone());
        sdf.field(q)
    }
}

/// Extrude a shape infinitely along an arbitrary axis.
pub type StretchInfinite<Dim, Sdf> = Operator<StretchInfiniteOp<Dim>, Sdf>;

impl<Dim, Sdf> StretchInfinite<Dim, Sdf> {
    pub fn dir(&mut self) -> &mut Dim {
        &mut self.op.dir
    }
}

/// Extrude a shape by an arbitrary distance along an arbitrary axis, preserving exterior geometry as caps.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct StretchDistOp<Dim> {
    pub dir: Dim,
    pub dist: f32,
}

impl Default for StretchDistOp<f32> {
    fn default() -> Self {
        StretchDistOp {
            dir: 1.0,
            dist: 1.0,
        }
    }
}

impl Default for StretchDistOp<Vec2> {
    fn default() -> Self {
        StretchDistOp {
            dir: Vec2::X,
            dist: 1.0,
        }
    }
}

impl Default for StretchDistOp<Vec3> {
    fn default() -> Self {
        StretchDistOp {
            dir: Vec3::X,
            dist: 1.0,
        }
    }
}

impl<Sdf, Dim, Attr> FieldOperator<Sdf, Attr> for StretchDistOp<Dim>
where
    Attr: Attribute<Input = Dim>,
    Sdf: Field<Attr>,
    Dim: Clone + Mul<f32, Output = Dim> + Sub<Dim, Output = Dim> + Dot,
{
    fn operator(&self, sdf: &Sdf, p: Dim) -> Attr::Output {
        let q =
            p.clone() - (self.dir.clone() * p.dot(self.dir.clone()).clamp(-self.dist, self.dist));
        sdf.field(q)
    }
}

/// Extrude a shape by an arbitrary distance along an arbitrary axis, preserving exterior geometry as caps.
pub type StretchDist<Dim, Sdf> = Operator<StretchDistOp<Dim>, Sdf>;

impl<Dim, Sdf> StretchDist<Dim, Sdf> {
    pub fn dir(&mut self) -> &mut Dim {
        &mut self.op.dir
    }

    pub fn dist(&mut self) -> &mut f32 {
        &mut self.op.dist
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test_stretch_infinite {
    use rust_gpu_bridge::glam::{Vec2, Vec3};
    use type_fields::field::Field;

    use crate::{
        prelude::{Cube, Point, StretchInfinite},
        test_op_attrs_1d, test_op_attrs_2d, test_op_attrs_3d,
    };

    #[test]
    fn test_stretch_infinite() {
        StretchInfinite::<_, Cube>::default().with(StretchInfinite::dir, Vec3::default());
    }

    test_op_attrs_1d!(StretchInfinite::<f32, Point>);
    test_op_attrs_2d!(StretchInfinite::<Vec2, Point>);
    test_op_attrs_3d!(StretchInfinite::<Vec3, Point>);
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test_stretch_dist {
    use rust_gpu_bridge::glam::{Vec2, Vec3};
    use type_fields::field::Field;

    use crate::{
        prelude::{Cube, Point, StretchDist},
        test_op_attrs_1d, test_op_attrs_2d, test_op_attrs_3d,
    };

    #[test]
    fn test_stretch_dist() {
        StretchDist::<_, Cube>::default()
            .with(StretchDist::dir, Vec3::default())
            .with(StretchDist::dist, f32::default());
    }

    test_op_attrs_1d!(StretchDist::<f32, Point>);
    test_op_attrs_2d!(StretchDist::<Vec2, Point>);
    test_op_attrs_3d!(StretchDist::<Vec3, Point>);
}
