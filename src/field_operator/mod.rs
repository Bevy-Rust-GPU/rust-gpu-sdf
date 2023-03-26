//! Types that modify a distance field.

pub mod axial_reflect;
pub mod colorize;
pub mod compose;
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
pub mod smooth_intersection;
pub mod smooth_subtraction;
pub mod smooth_union;
pub mod stretch;
pub mod subtraction;
pub mod sweep;
pub mod translate;
pub mod triplanar_uv;
pub mod twist;
pub mod union;

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
    Dim: Clone,
{
    fn evaluate(&self, attr: Attr, p: Dim) -> Attr::Type {
        self.op.operator(attr, &self.target, p)
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Point;

    use super::{isosurface::IsosurfaceOp, Operator};

    #[test]
    fn test_operator() {
        Operator::<IsosurfaceOp, Point>::default()
            .with(Operator::target, Point::default())
            .with(Operator::op, IsosurfaceOp::default());
    }
}

#[macro_export]
macro_rules! impl_passthrough_op_1 {
    ($ty:ty, <$pos: ident>, $attr:ty) => {
        crate::impl_passthrough_op!($ty, <$pos, Sdf>, Sdf, $attr);
    };
}

#[macro_export]
macro_rules! impl_passthrough_op_2 {
    ($ty:ty, <$pos: ident>, $attr:ty $(, $field:tt)?) => {
        crate::impl_passthrough_op!($ty, <$pos, SdfA, SdfB>, (SdfA, SdfB), $attr $(, $field)?);
    };
}

#[macro_export]
macro_rules! impl_passthrough_op {
    ($ty:ty, <$pos: ident, $($sdf_gen:ident),+>, $sdf_ty:ty, $attr:ty $(, $field:tt)?) => {
        impl<$($sdf_gen),+, $pos> FieldOperator<$sdf_ty, $pos, $attr> for $ty
        where
            $attr: crate::prelude::Attribute,
            $($sdf_gen: crate::prelude::FieldFunction<$pos, $attr>),+
        {
            fn operator(
                &self,
                attr: $attr,
                sdf: &$sdf_ty,
                p: $pos,
            ) -> <$attr as crate::prelude::Attribute>::Type {
                sdf$(.$field)?.evaluate(attr, p)
            }
        }
    };
}
