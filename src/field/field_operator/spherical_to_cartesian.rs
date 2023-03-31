use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Cos, Sin,
};

use crate::prelude::{Attribute, Field};

use super::{FieldOperator, Operator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SphericalToCartesianOp;

impl<Sdf, Attr> FieldOperator<Sdf, Vec2, Attr> for SphericalToCartesianOp
where
    Sdf: Field<Vec2, Attr>,
    Attr: Attribute,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: Vec2) -> Attr::Type {
        let x = p.y * p.x.cos();
        let y = p.y * p.x.sin();
        sdf.field(attr, Vec2::new(y, x))
    }
}

impl<Sdf, Attr> FieldOperator<Sdf, Vec3, Attr> for SphericalToCartesianOp
where
    Sdf: Field<Vec3, Attr>,
    Attr: Attribute,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: Vec3) -> Attr::Type {
        let x = p.y * p.x.sin() * p.z.cos();
        let y = p.y * p.x.sin() * p.z.sin();
        let z = p.y * p.x.cos();
        sdf.field(attr, Vec3::new(x, y, z))
    }
}

pub type SphericalToCartesian<Sdf> = Operator<SphericalToCartesianOp, Sdf>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{
        prelude::{Point, SphericalToCartesian},
        test_op_attrs_2d, test_op_attrs_3d,
    };

    #[test]
    fn test_spherical_to_cartesian() {
        SphericalToCartesian::<Point>::default();
    }

    test_op_attrs_2d!(SphericalToCartesian::<Point>);
    test_op_attrs_3d!(SphericalToCartesian::<Point>);
}
