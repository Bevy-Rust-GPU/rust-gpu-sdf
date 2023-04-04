use rust_gpu_bridge::{
    glam::{Vec2, Vec3, Vec3Swizzles},
    Acos, Atan2, Sign,
};

use crate::prelude::{Attribute, Field, FieldOperator, Operator, items::position::Position};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct CartesianToPolarOp;

impl<Sdf, Attr> FieldOperator<Sdf, Attr> for CartesianToPolarOp
where
    Sdf: Field<Attr>,
    Attr: Attribute<Input = Position<Vec2>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec2>) -> Attr::Output {
        let r = p.length();
        let a = p.x.atan2(p.y);
        sdf.field(&Vec2::new(a, r).into())
    }
}

pub type CartesianToPolar<Sdf> = Operator<CartesianToPolarOp, Sdf>;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct CartesianToSphericalOp;

impl<Sdf, Attr> FieldOperator<Sdf, Attr> for CartesianToSphericalOp
where
    Sdf: Field<Attr>,
    Attr: Attribute<Input = Position<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec3>) -> Attr::Output {
        let r = p.length();
        let i = (p.z / r).acos();
        let a = p.y.sign() * (p.x / p.xy().length()).acos();
        sdf.field(&Vec3::new(i, r, a).into())
    }
}

pub type CartesianToSpherical<Sdf> = Operator<CartesianToSphericalOp, Sdf>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{
        prelude::{CartesianToSpherical, Point},
        test_op_attrs_2d, test_op_attrs_3d,
    };

    use super::CartesianToPolar;

    #[test]
    fn test_cartesian_to_polar() {
        CartesianToPolar::<Point>::default();
    }

    #[test]
    fn test_cartesian_to_spherical() {
        CartesianToSpherical::<Point>::default();
    }

    test_op_attrs_2d!(CartesianToPolar::<Point>);
    test_op_attrs_3d!(CartesianToSpherical::<Point>);
}
