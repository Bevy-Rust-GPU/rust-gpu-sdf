//! Compute the blended boolean intersection of two distance fields.

use core::ops::{Add, Div, Mul, Sub};

use rust_gpu_bridge::{glam::Vec2, Clamp, Mix, Normalize, Saturate, Splat, Step};
use type_fields::Field;

use crate::prelude::{Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv};

/// Compute the blended boolean intersection of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct SmoothIntersectionOp {
    pub k: f32,
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Distance<Dim>> for SmoothIntersectionOp
where
    SdfA: Field<Distance<Dim>>,
    SdfB: Field<Distance<Dim>>,
    Dim: Clone,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> f32 {
        let d1 = sdf_a.field(p.clone());
        let d2 = sdf_b.field(p);
        let h = (0.5 - 0.5 * (d2 - d1) / self.k).clamp(0.0, 1.0);
        d2.mix(d1, h).add(self.k.mul(h).mul(1.0 - h)).into()
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Normal<Dim>> for SmoothIntersectionOp
where
    SdfA: Field<Distance<Dim>>,
    SdfA: Field<Normal<Dim>>,
    SdfB: Field<Distance<Dim>>,
    SdfB: Field<Normal<Dim>>,
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
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> Dim {
        let d1 = Field::<Distance<Dim>>::field(sdf_a, p.clone());
        let d2 = Field::<Distance<Dim>>::field(sdf_b, p.clone());

        let h = (d2.clone() - d1.clone())
            .div(self.k)
            .mul(0.5)
            .sub(0.5)
            .saturate();

        let n1 = Field::<Normal<Dim>>::field(sdf_a, p.clone());
        let n2 = Field::<Normal<Dim>>::field(sdf_b, p.clone());

        n2.mix(n1, Dim::splat(h.clone())).normalize()
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Tangent<Dim>> for SmoothIntersectionOp
where
    SdfA: Field<Distance<Dim>>,
    SdfA: Field<Tangent<Dim>>,
    SdfB: Field<Distance<Dim>>,
    SdfB: Field<Tangent<Dim>>,
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
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> Dim {
        let d1 = Field::<Distance<Dim>>::field(sdf_a, p.clone());
        let d2 = Field::<Distance<Dim>>::field(sdf_b, p.clone());

        let h = (d2.clone() - d1.clone())
            .div(self.k)
            .mul(0.5)
            .sub(0.5)
            .saturate();

        let n1 = Field::<Tangent<Dim>>::field(sdf_a, p.clone());
        let n2 = Field::<Tangent<Dim>>::field(sdf_b, p.clone());

        n2.mix(n1, Dim::splat(h.clone())).normalize()
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Uv<Dim>> for SmoothIntersectionOp
where
    SdfA: Field<Distance<Dim>>,
    SdfA: Field<Uv<Dim>>,
    SdfB: Field<Distance<Dim>>,
    SdfB: Field<Uv<Dim>>,
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
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> Vec2 {
        let d1 = Field::<Distance<Dim>>::field(sdf_a, p.clone());
        let d2 = Field::<Distance<Dim>>::field(sdf_b, p.clone());

        let h = (d2.clone() - d1.clone())
            .div(self.k)
            .mul(0.5)
            .sub(0.5)
            .saturate();

        let uv1 = Field::<Uv<Dim>>::field(sdf_a, p.clone());
        let uv2 = Field::<Uv<Dim>>::field(sdf_b, p.clone());

        uv2.mix(uv1, Vec2::splat(h.step(0.5)))
    }
}

/// Compute the blended boolean intersection of two distance fields.
pub type SmoothIntersection<SdfA, SdfB> = Operator<SmoothIntersectionOp, (SdfA, SdfB)>;

impl<SdfA, SdfB> SmoothIntersection<SdfA, SdfB> {
    pub fn sdf_a(&mut self) -> &mut SdfA {
        &mut self.target().0
    }

    pub fn sdf_b(&mut self) -> &mut SdfB {
        &mut self.target().1
    }

    pub fn k(&mut self) -> &mut f32 {
        self.op().k()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use type_fields::field::Field;

    use crate::{
        prelude::{Cube, Point, SmoothIntersection, Sphere},
        test_op_attrs,
    };

    #[test]
    fn test_smooth_intersection() {
        SmoothIntersection::<Cube, Sphere>::default().with(SmoothIntersection::k, f32::default());
    }

    test_op_attrs!(SmoothIntersection::<Point, Point>);
}
