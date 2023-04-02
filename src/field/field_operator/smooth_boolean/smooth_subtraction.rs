//! Compute the blended boolean subtraction of two distance fields.
use core::ops::{Add, Div, Mul, Sub};

use rust_gpu_bridge::{glam::Vec2, Clamp, Mix, Normalize, Saturate, Splat, Step};
use type_fields::Field;

use crate::prelude::{Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv};

/// Compute the blended boolean subtraction of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct SmoothSubtractionOp {
    pub k: f32,
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Distance> for SmoothSubtractionOp
where
    SdfA: Field<Dim, Distance>,
    SdfB: Field<Dim, Distance>,
    Dim: Clone,
{
    fn operator(&self, attr: Distance, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> f32 {
        let d1 = sdf_a.field(attr, p.clone());
        let d2 = sdf_b.field(attr, p);
        let h = (0.5 - 0.5 * (d2 + d1) / self.k).clamp(0.0, 1.0);
        d2.mix(-d1, h).add(self.k.mul(h).mul(1.0 - h)).into()
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Normal<Dim>> for SmoothSubtractionOp
where
    SdfA: Field<Dim, Distance>,
    SdfA: Field<Dim, Normal<Dim>>,
    SdfB: Field<Dim, Distance>,
    SdfB: Field<Dim, Normal<Dim>>,
    Dim: Clone
        + Sub<Dim, Output = Dim>
        + Div<f32, Output = Dim>
        + Mul<f32, Output = Dim>
        + Mul<Dim, Output = Dim>
        + Add<f32, Output = Dim>
        + Add<Dim, Output = Dim>
        + Clamp
        + Mix
        + Saturate
        + Normalize
        + Splat,
{
    fn operator(&self, attr: Normal<Dim>, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> Dim {
        let d1 = sdf_a.field(Distance, p.clone());
        let d2 = sdf_b.field(Distance, p.clone());

        let h = (d2.clone() + d1.clone())
            .div(self.k)
            .mul(0.5)
            .sub(0.5)
            .saturate();

        let n1 = sdf_a.field(attr, p.clone());
        let n2 = sdf_b.field(attr, p);

        n2.mix(n1.mul(-1.0), Dim::splat(h)).normalize()
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Tangent<Dim>> for SmoothSubtractionOp
where
    SdfA: Field<Dim, Distance>,
    SdfA: Field<Dim, Tangent<Dim>>,
    SdfB: Field<Dim, Distance>,
    SdfB: Field<Dim, Tangent<Dim>>,
    Dim: Clone
        + Sub<Dim, Output = Dim>
        + Div<f32, Output = Dim>
        + Mul<f32, Output = Dim>
        + Mul<Dim, Output = Dim>
        + Add<Dim, Output = Dim>
        + Add<f32, Output = Dim>
        + Clamp
        + Mix
        + Saturate
        + Normalize
        + Splat,
{
    fn operator(&self, attr: Tangent<Dim>, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> Dim {
        let d1 = sdf_a.field(Distance, p.clone());
        let d2 = sdf_b.field(Distance, p.clone());

        let h = (d2.clone() + d1.clone())
            .div(self.k)
            .mul(0.5)
            .sub(0.5)
            .saturate();

        let n1 = sdf_a.field(attr, p.clone());
        let n2 = sdf_b.field(attr, p);

        n2.mix(n1.mul(-1.0), Dim::splat(h)).normalize()
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Dim, Uv> for SmoothSubtractionOp
where
    SdfA: Field<Dim, Distance>,
    SdfA: Field<Dim, Uv>,
    SdfB: Field<Dim, Distance>,
    SdfB: Field<Dim, Uv>,
    Dim: Clone
        + Sub<Dim, Output = Dim>
        + Div<f32, Output = Dim>
        + Mul<f32, Output = Dim>
        + Mul<Dim, Output = Dim>
        + Add<Dim, Output = Dim>
        + Add<f32, Output = Dim>
        + Clamp
        + Mix
        + Saturate
        + Normalize
        + Splat,
{
    fn operator(&self, attr: Uv, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> Vec2 {
        let d1 = sdf_a.field(Distance, p.clone());
        let d2 = sdf_b.field(Distance, p.clone());

        let h = (d2.clone() + d1.clone())
            .div(self.k)
            .mul(0.5)
            .sub(0.5)
            .saturate();

        let n1 = sdf_a.field(attr, p.clone());
        let n2 = sdf_b.field(attr, p);

        n2.mix(n1.mul(-1.0), Vec2::splat(h.step(0.5)))
    }
}

/// Compute the blended boolean subtraction of two distance fields.
pub type SmoothSubtraction<SdfA, SdfB> = Operator<SmoothSubtractionOp, (SdfA, SdfB)>;

impl<SdfA, SdfB> SmoothSubtraction<SdfA, SdfB> {
    pub fn sdf_a(&mut self) -> &mut SdfA {
        &mut self.target().0
    }

    pub fn sdf_b(&mut self) -> &mut SdfB {
        &mut self.target().1
    }

    pub fn k(&mut self) -> &mut f32 {
        &mut self.op.k
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use type_fields::field::Field;

    use crate::{
        prelude::{Cube, Point, SmoothSubtraction, Sphere},
        test_op_attrs,
    };

    #[test]
    fn test_smooth_subtraction() {
        SmoothSubtraction::<Cube, Sphere>::default().with(SmoothSubtraction::k, f32::default());
    }

    test_op_attrs!(SmoothSubtraction::<Point, Point>);
}
