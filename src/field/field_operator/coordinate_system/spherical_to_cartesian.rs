use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Cos, Sin,
};

use crate::prelude::{Attribute, Field, FieldOperator, Operator, items::position::Position};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct PolarToCartesianOp;

impl<Sdf, Attr> FieldOperator<Sdf, Attr> for PolarToCartesianOp
where
    Sdf: Field<Attr>,
    Attr: Attribute<Input = Position<Vec2>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec2>) -> Attr::Output {
        let x = p.y * p.x.cos();
        let y = p.y * p.x.sin();
        sdf.field(&Vec2::new(y, x).into())
    }
}

pub type PolarToCartesian<Sdf> = Operator<PolarToCartesianOp, Sdf>;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct SphericalToCartesianOp;

impl<Sdf, Attr> FieldOperator<Sdf, Attr> for SphericalToCartesianOp
where
    Sdf: Field<Attr>,
    Attr: Attribute<Input = Position<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec3>) -> Attr::Output {
        let x = p.y * p.x.sin() * p.z.cos();
        let y = p.y * p.x.sin() * p.z.sin();
        let z = p.y * p.x.cos();
        sdf.field(&Vec3::new(x, y, z).into())
    }
}

pub type SphericalToCartesian<Sdf> = Operator<SphericalToCartesianOp, Sdf>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{
        prelude::{Point, SphericalToCartesian},
        test_op_attrs_2d, test_op_attrs_3d,
    };

    use super::PolarToCartesian;

    #[test]
    fn test_polar_to_cartesian() {
        PolarToCartesian::<Point>::default();
    }

    #[test]
    fn test_spherical_to_cartesian() {
        SphericalToCartesian::<Point>::default();
    }

    test_op_attrs_2d!(PolarToCartesian::<Point>);
    test_op_attrs_3d!(SphericalToCartesian::<Point>);
}
