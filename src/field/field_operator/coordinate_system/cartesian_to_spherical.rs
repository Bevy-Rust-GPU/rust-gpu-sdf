use rust_gpu_bridge::{
    glam::{Vec2, Vec3, Vec3Swizzles},
    Acos, Atan2, Sign,
};

use crate::prelude::{Attribute, Field, FieldOperator, Operator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct CartesianToSphericalOp;

impl<Sdf, Attr> FieldOperator<Sdf, Vec2, Attr> for CartesianToSphericalOp
where
    Sdf: Field<Vec2, Attr>,
    Attr: Attribute,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: Vec2) -> Attr::Type {
        let r = p.length();
        let a = p.x.atan2(p.y);
        sdf.field(attr, Vec2::new(a, r))
    }
}

impl<Sdf, Attr> FieldOperator<Sdf, Vec3, Attr> for CartesianToSphericalOp
where
    Sdf: Field<Vec3, Attr>,
    Attr: Attribute,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: Vec3) -> Attr::Type {
        let r = p.length();
        let i = (p.z / r).acos();
        let a = p.y.sign() * (p.x / p.xy().length()).acos();
        sdf.field(attr, Vec3::new(i, r, a))
    }
}

pub type CartesianToSpherical<Sdf> = Operator<CartesianToSphericalOp, Sdf>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{
        prelude::{CartesianToSpherical, Point},
        test_op_attrs_2d, test_op_attrs_3d,
    };

    #[test]
    fn test_cartesian_to_spherical() {
        CartesianToSpherical::<Point>::default();
    }

    test_op_attrs_2d!(CartesianToSpherical::<Point>);
    test_op_attrs_3d!(CartesianToSpherical::<Point>);
}
