//! Types that modify a distance field.

pub mod axial_reflect;
pub mod cartesian_to_spherical;
pub mod colorize;
pub mod composite;
pub mod conditional;
pub mod displace;
pub mod elongate;
pub mod extrude;
pub mod extrude_interior;
pub mod gradient_central_diff;
pub mod gradient_tetrahedron;
pub mod gradient_uv;
pub mod hollow;
pub mod intersection;
pub mod isosurface;
pub mod normalize;
pub mod reflect;
pub mod repeat;
pub mod rotate;
pub mod scale;
pub mod sdf_color;
pub mod sdf_normal;
pub mod sdf_tangent;
pub mod sdf_uv;
pub mod sided;
pub mod slice;
pub mod smooth_intersection;
pub mod smooth_subtraction;
pub mod smooth_union;
pub mod spherical_to_cartesian;
pub mod stretch;
pub mod subtraction;
pub mod sweep;
pub mod translate;
pub mod triplanar_uv;
pub mod twist;
pub mod union;

pub mod raycast;

use crate::prelude::{Attribute, FieldFunction};

/// Modifies the input / output of a [`FieldFunction`].
pub trait FieldOperator<Sdf, Pos, Attr>
where
    Attr: Attribute,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: Pos) -> Attr::Type;
}

impl<Sdf, Pos, Attr> FieldOperator<Sdf, Pos, Attr> for ()
where
    Attr: Attribute,
    Sdf: FieldFunction<Pos, Attr>,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: Pos) -> Attr::Type {
        sdf.evaluate(attr, p)
    }
}

/// Applies a [`FieldOperator`] to a [`FieldFunction`].
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Hash, type_fields::Field)]
#[repr(C)]
pub struct Operator<Op, Sdf> {
    pub target: Sdf,
    pub op: Op,
}

impl<Op, Sdf, Dim, Attr> FieldFunction<Dim, Attr> for Operator<Op, Sdf>
where
    Attr: Attribute,
    Op: FieldOperator<Sdf, Dim, Attr>,
{
    fn evaluate(&self, attr: Attr, p: Dim) -> Attr::Type {
        self.op.operator(attr, &self.target, p)
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

    use crate::prelude::{IsosurfaceOp, Operator, Point};

    #[test]
    fn test_operator() {
        Operator::<IsosurfaceOp, Point>::default()
            .with(Operator::target, Point::default())
            .with(Operator::op, IsosurfaceOp::default());
    }
}

#[macro_export]
macro_rules! impl_passthrough_op_1 {
    ($ty:ty, $attr:ty, $pos:ident $($gen:tt)*) => {
        impl<Sdf, $pos $($gen)*> FieldOperator<Sdf, $pos, $attr> for $ty
        where
            $attr: crate::prelude::Attribute,
            Sdf: crate::prelude::FieldFunction<$pos, $attr>,
        {
            fn operator(
                &self,
                attr: $attr,
                sdf: &Sdf,
                p: $pos,
            ) -> <$attr as crate::prelude::Attribute>::Type {
                sdf.evaluate(attr, p)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_passthrough_op_2 {
    ($ty:ty, $attr:ty, $field:tt, $pos:ident $($gen:tt)*) => {
        impl<SdfA, SdfB, $pos $($gen)*> FieldOperator<(SdfA, SdfB), $pos, $attr> for $ty
        where
            $attr: crate::prelude::Attribute,
            SdfA: crate::prelude::FieldFunction<$pos, $attr>,
            SdfB: crate::prelude::FieldFunction<$pos, $attr>,
        {
            fn operator(
                &self,
                attr: $attr,
                sdf: &(SdfA, SdfB),
                p: $pos,
            ) -> <$attr as crate::prelude::Attribute>::Type {
                sdf.$field.evaluate(attr, p)
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
                let _ = crate::prelude::FieldFunction::evaluate(&f, <$attrs>::default(), <$pos>::default());
            )*
        }
    };
}
