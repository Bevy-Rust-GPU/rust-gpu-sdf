//! Types that modify a distance field.

pub mod checker;
pub mod color_normal;
pub mod color_tangent;
pub mod color_uv;
pub mod composite;
pub mod displace;
pub mod displace_proxy;
pub mod elongate;
pub mod hollow;
pub mod isosurface;
pub mod isosurface_proxy;
pub mod normalize;
pub mod scale_uv;
pub mod sided;
pub mod stretch;
pub mod triplanar_uv;
pub mod twist;

pub mod arity;
pub mod boolean;
pub mod coordinate_system;
pub mod gradient;
pub mod proxy;
pub mod raycast;
pub mod reflect;
pub mod repeat;
pub mod smooth_boolean;
pub mod transform;

use crate::prelude::{Attribute, Field};

/// Modifies the input / output of a [`FieldAttribute`].
pub trait FieldOperator<Sdf, Attr>
where
    Attr: Attribute,
{
    fn operator(&self, sdf: &Sdf, p: Attr::Input) -> Attr::Output;
}

/// Applies a [`FieldOperator`] to a [`FieldAttribute`].
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Hash, type_fields::Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[cfg_attr(feature = "bevy", derive(bevy::reflect::TypeUuid))]
#[cfg_attr(feature = "bevy", uuid = "d588f817-4e15-4b1e-b98c-dc2b0d47f719")]
#[repr(C)]
pub struct Operator<Op, Sdf> {
    pub target: Sdf,
    pub op: Op,
}

impl<Op, Sdf, Attr> Field<Attr> for Operator<Op, Sdf>
where
    Op: FieldOperator<Sdf, Attr>,
    Attr: Attribute,
{
    fn field(&self, p: Attr::Input) -> Attr::Output {
        self.op.operator(&self.target, p)
    }
}

#[cfg(feature = "glam")]
pub mod boxed {
    extern crate alloc;
    use alloc::boxed::Box;

    use crate::prelude::{Attribute, FieldOperator};

    impl<Sdf, Attr> FieldOperator<Sdf, Attr> for Box<dyn FieldOperator<Sdf, Attr>>
    where
        Attr: Attribute,
    {
        fn operator(
            &self,
            sdf: &Sdf,
            p: <Attr as Attribute>::Input,
        ) -> <Attr as Attribute>::Output {
            self.as_ref().operator(sdf, p)
        }
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use type_fields::field::Field;

    use crate::prelude::{IsosurfaceProxyOp, Operator, Point};

    #[test]
    fn test_operator() {
        Operator::<IsosurfaceProxyOp, Point>::default()
            .with(Operator::target, Point::default())
            .with(Operator::op, IsosurfaceProxyOp::default());
    }
}

#[macro_export]
macro_rules! impl_passthrough_op_1 {
    ($ty:ty, $attr:ty, $($gen:tt)*) => {
        impl<Sdf, $($gen)*> FieldOperator<Sdf, $attr> for $ty
        where
            Sdf: crate::prelude::Field<$attr>,
        {
            fn operator(
                &self,
                sdf: &Sdf,
                p: <$attr as crate::prelude::Attribute>::Input,
            ) -> <$attr as crate::prelude::Attribute>::Output {
                sdf.field(p)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_passthrough_op_2 {
    ($ty:ty, $attr:ty, $field:tt, $sdf:ident $($gen:tt)*) => {
        impl<SdfA, SdfB $($gen)*> FieldOperator<(SdfA, SdfB), $attr> for $ty
        where
            $sdf: crate::prelude::Field<$attr>,
        {
            fn operator(
                &self,
                sdf: &(SdfA, SdfB),
                p: <$attr as crate::prelude::Attribute>::Input,
            ) -> <$attr as crate::prelude::Attribute>::Output {
                sdf.$field.field(p)
            }
        }
    };
}

#[macro_export]
macro_rules! test_op_attrs {
    ($ty:ty) => {
        crate::test_op_attrs_1d!($ty);
        crate::test_op_attrs_2d!($ty);
        crate::test_op_attrs_3d!($ty);
    };
}

#[macro_export]
macro_rules! test_op_attrs_1d {
    ($ty:ty) => {
        crate::test_op_attrs_impl!($ty, test_attrs_1d, f32, [crate::prelude::Distance<f32>, crate::prelude::Normal<f32>, crate::prelude::Uv<f32>]);
    };
}

#[macro_export]
macro_rules! test_op_attrs_2d {
    ($ty:ty) => {
        crate::test_op_attrs_impl!($ty, test_attrs_2d, rust_gpu_bridge::glam::Vec2, [crate::prelude::Distance<rust_gpu_bridge::glam::Vec2>, crate::prelude::Normal<rust_gpu_bridge::glam::Vec2>, crate::prelude::Uv<rust_gpu_bridge::glam::Vec2>]);
    };
}

#[macro_export]
macro_rules! test_op_attrs_3d {
    ($ty:ty) => {
        crate::test_op_attrs_impl!($ty, test_attrs_3d, rust_gpu_bridge::glam::Vec3, [crate::prelude::Distance<rust_gpu_bridge::glam::Vec3>, crate::prelude::Normal<rust_gpu_bridge::glam::Vec3>, crate::prelude::Uv<rust_gpu_bridge::glam::Vec3>]);
    };
}

#[macro_export]
macro_rules! test_op_attrs_impl {
    ($ty:ty, $ident:ident, $pos:ty, [$($attrs:ty),+]) => {
        #[test]
        fn $ident() {
            let f = <$ty>::default();
            $(
                let _ = crate::prelude::Field::<$attrs>::field(&f, <$pos>::default());
            )*
        }
    };
}
