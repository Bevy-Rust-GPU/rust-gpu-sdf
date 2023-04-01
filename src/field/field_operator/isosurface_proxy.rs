//! Shift the isosurface of a distance field by a given amount.

use core::ops::Div;

use type_fields::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{
        Color, DisplaceProxyOp, Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv,
    },
};

/// Shift the isosurface of a distance field by a given amount.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct IsosurfaceProxyOp;

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Distance> for IsosurfaceProxyOp
where
    SdfA: Field<Dim, Distance>,
    SdfB: Field<Dim, Distance>,
    Dim: Clone,
{
    fn operator(&self, attr: Distance, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> f32 {
        let d1 = sdf_a.field(attr, p.clone());
        let d2 = sdf_b.field(attr, p);
        d1 - d2
    }
}

impl_passthrough_op_2!(IsosurfaceProxyOp, Normal<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(IsosurfaceProxyOp, Tangent<Dim>, 0, SdfA, Dim);

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Uv> for IsosurfaceProxyOp
where
    Uv: crate::prelude::Attribute,
    SdfA: crate::prelude::Field<Dim, Uv>,
    SdfB: crate::prelude::Field<Dim, Distance>,
    Dim: Clone + Div<f32, Output = Dim>,
{
    fn operator(
        &self,
        attr: Uv,
        (sdf_a, sdf_b): &(SdfA, SdfB),
        p: Dim,
    ) -> <Uv as crate::prelude::Attribute>::Type {
        let p = p.clone() / sdf_b.field(Distance, p);
        sdf_a.field(attr, p)
    }
}

impl_passthrough_op_2!(IsosurfaceProxyOp, Color, 0, SdfA, Dim);

/// Add an arbitrary radius to a distance field.
pub type IsosurfaceProxy<SdfA, SdfB> = Operator<DisplaceProxyOp, (SdfA, SdfB)>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{
        prelude::{IsosurfaceProxy, Point},
        test_op_attrs,
    };

    #[test]
    fn test_isosurface() {
        IsosurfaceProxy::<Point>::default();
    }

    test_op_attrs!(IsosurfaceProxy::<Point>);
}
