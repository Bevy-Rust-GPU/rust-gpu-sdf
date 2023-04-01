//! Types that modify a distance field.

pub mod composite;
pub mod displace;
pub mod displace_proxy;
pub mod elongate;
pub mod hollow;
pub mod isosurface;
pub mod isosurface_proxy;
pub mod normalize;
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
pub trait FieldOperator<Sdf, Pos, Attr>
where
    Attr: Attribute,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: Pos) -> Attr::Type;
}

/// Applies a [`FieldOperator`] to a [`FieldAttribute`].
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Hash, type_fields::Field)]
#[cfg_attr(feature = "bevy", derive(bevy::reflect::TypeUuid))]
#[cfg_attr(feature = "bevy", uuid = "d588f817-4e15-4b1e-b98c-dc2b0d47f719")]
#[repr(C)]
pub struct Operator<Op, Sdf> {
    pub target: Sdf,
    pub op: Op,
}

impl<Op, Sdf, Dim, Attr> Field<Dim, Attr> for Operator<Op, Sdf>
where
    Op: FieldOperator<Sdf, Dim, Attr>,
    Attr: Attribute,
{
    fn field(&self, attr: Attr, p: Dim) -> Attr::Type {
        self.op.operator(attr, &self.target, p)
    }
}

#[cfg(feature = "glam")]
use rust_gpu_bridge::{format, Named, String, ToString};

#[cfg(feature = "glam")]
impl<Op, Sdf> Named for Operator<Op, Sdf>
where
    Op: Named,
    Sdf: Named,
{
    fn module() -> String {
        module_path!().to_string()
    }

    fn short_name() -> String {
        format!("Operator<{}, {}>", Op::short_name(), Sdf::short_name())
    }

    fn name() -> String {
        format!(
            "{}::Operator<{}, {}>",
            Self::module(),
            Op::name(),
            Sdf::name()
        )
    }
}

#[cfg(feature = "glam")]
pub mod boxed {
    extern crate alloc;
    use alloc::boxed::Box;

    use crate::prelude::{Attribute, FieldOperator};

    impl<Sdf, Dim, Attr> FieldOperator<Sdf, Dim, Attr> for Box<dyn FieldOperator<Sdf, Dim, Attr>>
    where
        Attr: Attribute,
    {
        fn operator(&self, attr: Attr, sdf: &Sdf, p: Dim) -> <Attr as Attribute>::Type {
            self.as_ref().operator(attr, sdf, p)
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
    ($ty:ty, $attr:ty, $pos:ident $($gen:tt)*) => {
        impl<Sdf, $pos $($gen)*> FieldOperator<Sdf, $pos, $attr> for $ty
        where
            Sdf: crate::prelude::Field<$pos, $attr>,
        {
            fn operator(
                &self,
                attr: $attr,
                sdf: &Sdf,
                p: $pos,
            ) -> <$attr as crate::prelude::Attribute>::Type {
                sdf.field(attr, p)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_passthrough_op_2 {
    ($ty:ty, $attr:ty, $field:tt, $sdf:ident, $pos:ident $($gen:tt)*) => {
        impl<SdfA, SdfB, $pos $($gen)*> FieldOperator<(SdfA, SdfB), $pos, $attr> for $ty
        where
            $sdf: crate::prelude::Field<$pos, $attr>,
        {
            fn operator(
                &self,
                attr: $attr,
                sdf: &(SdfA, SdfB),
                p: $pos,
            ) -> <$attr as crate::prelude::Attribute>::Type {
                sdf.$field.field(attr, p)
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
        crate::test_op_attrs_impl!($ty, test_attrs_1d, f32, [crate::prelude::Distance, crate::prelude::Normal<f32>, crate::prelude::Uv]);
    };
}

#[macro_export]
macro_rules! test_op_attrs_2d {
    ($ty:ty) => {
        crate::test_op_attrs_impl!($ty, test_attrs_2d, rust_gpu_bridge::glam::Vec2, [crate::prelude::Distance, crate::prelude::Normal<rust_gpu_bridge::glam::Vec2>, crate::prelude::Uv]);
    };
}

#[macro_export]
macro_rules! test_op_attrs_3d {
    ($ty:ty) => {
        crate::test_op_attrs_impl!($ty, test_attrs_3d, rust_gpu_bridge::glam::Vec3, [crate::prelude::Distance, crate::prelude::Normal<rust_gpu_bridge::glam::Vec3>, crate::prelude::Uv]);
    };
}

#[macro_export]
macro_rules! test_op_attrs_impl {
    ($ty:ty, $ident:ident, $pos:ty, [$($attrs:ty),+]) => {
        #[test]
        fn $ident() {
            let f = <$ty>::default();
            $(
                let _ = crate::prelude::Field::field(&f, <$attrs>::default(), <$pos>::default());
            )*
        }
    };
}
