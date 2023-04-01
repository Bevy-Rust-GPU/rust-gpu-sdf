//! Shift the isosurface of a distance field by a given amount.

use core::ops::Div;

use type_fields::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{Color, Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv},
};

/// Shift the isosurface of a distance field by a given amount.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "bevy", derive(bevy::reflect::TypeUuid))]
#[cfg_attr(feature = "bevy", uuid = "d588f817-4e15-4b1e-b98c-dc2b0d47f719")]
#[repr(C)]
pub struct IsosurfaceOp {
    pub delta: f32,
}

impl Default for IsosurfaceOp {
    fn default() -> Self {
        IsosurfaceOp { delta: 1.0 }
    }
}

impl<SdfA, Dim> FieldOperator<SdfA, Dim, Distance> for IsosurfaceOp
where
    SdfA: Field<Dim, Distance>,
    Dim: Clone,
{
    fn operator(&self, attr: Distance, sdf_a: &SdfA, p: Dim) -> f32 {
        sdf_a.field(attr, p.clone()) - self.delta
    }
}

impl_passthrough_op_2!(IsosurfaceOp, Normal<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(IsosurfaceOp, Tangent<Dim>, 0, SdfA, Dim);

impl<SdfA, Dim> FieldOperator<SdfA, Dim, Uv> for IsosurfaceOp
where
    Uv: crate::prelude::Attribute,
    SdfA: crate::prelude::Field<Dim, Uv>,
    Dim: Clone + Div<f32, Output = Dim>,
{
    fn operator(&self, attr: Uv, sdf_a: &SdfA, p: Dim) -> <Uv as crate::prelude::Attribute>::Type {
        let p = p.clone() / self.delta;
        sdf_a.field(attr, p)
    }
}

impl_passthrough_op_2!(IsosurfaceOp, Color, 0, SdfA, Dim);

#[cfg(feature = "bevy")]
use rust_gpu_bridge::{Named, String, ToString, format};

#[cfg(feature = "bevy")]
impl Named for IsosurfaceOp {
    fn module() -> String {
        module_path!().to_string()
    }

    fn short_name() -> String {
        "IsosurfaceOp".to_string()
    }
}

/// Add an arbitrary radius to a distance field.
pub type Isosurface<SdfA> = Operator<IsosurfaceOp, SdfA>;

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
