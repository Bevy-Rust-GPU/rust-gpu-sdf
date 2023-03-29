//! Shift the isosurface of a distance field by a given amount.

use core::ops::{Div, Sub};

use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, FieldFunction, FieldOperator, Normal, Operator, Tangent, Uv},
};

/// Shift the isosurface of a distance field by a given amount.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct IsosurfaceOp {
    pub delta: f32,
}

impl Default for IsosurfaceOp {
    fn default() -> Self {
        IsosurfaceOp { delta: 1.0 }
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Distance> for IsosurfaceOp
where
    Sdf: FieldFunction<Dim, Distance>,
{
    fn operator(&self, attr: Distance, sdf: &Sdf, p: Dim) -> f32 {
        sdf.evaluate(attr, p).sub(self.delta)
    }
}

impl_passthrough_op_1!(IsosurfaceOp, <Dim>, Normal<Dim>);
impl_passthrough_op_1!(IsosurfaceOp, <Dim>, Tangent<Dim>);

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Uv> for IsosurfaceOp
where
    Uv: crate::prelude::Attribute,
    Sdf: crate::prelude::FieldFunction<Dim, Uv>,
    Dim: Div<f32, Output = Dim>,
{
    fn operator(&self, attr: Uv, sdf: &Sdf, p: Dim) -> <Uv as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p / self.delta)
    }
}

impl_passthrough_op_1!(IsosurfaceOp, <Dim>, Color);

/// Add an arbitrary radius to a distance field.
pub type Isosurface<Sdf> = Operator<IsosurfaceOp, Sdf>;

impl<Sdf> Isosurface<Sdf> {
    pub fn delta(&mut self) -> &mut f32 {
        &mut self.op.delta
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use type_fields::field::Field;

    use crate::{
        prelude::{Isosurface, Point},
        test_op_attrs,
    };

    #[test]
    fn test_isosurface() {
        Isosurface::<Point>::default().with(Isosurface::delta, f32::default());
    }

    test_op_attrs!(Isosurface::<Point>);
}