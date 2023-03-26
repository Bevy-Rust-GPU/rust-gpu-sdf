use rust_gpu_bridge::glam::{Vec2, Vec4};

use crate::prelude::{Color, Distance, FieldFunction, Normal, Tangent, Uv};

use super::{Operator, SignedDistanceOperator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct NormalizeOp;

impl NormalizeOp {}

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Distance> for NormalizeOp
where
    Sdf: FieldFunction<Dim, Distance>,
{
    fn operator(&self, attr: Distance, sdf: &Sdf, p: Dim) -> f32 {
        sdf.evaluate(attr, p)
    }
}

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Normal<Dim>> for NormalizeOp
where
    Sdf: FieldFunction<Dim, Normal<Dim>>,
    Dim: Clone + rust_gpu_bridge::Normalize,
{
    fn operator(&self, attr: Normal<Dim>, sdf: &Sdf, p: Dim) -> Dim {
        sdf.evaluate(attr, p).clone().normalize()
    }
}

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Tangent<Dim>> for NormalizeOp
where
    Sdf: FieldFunction<Dim, Tangent<Dim>>,
    Dim: Clone + rust_gpu_bridge::Normalize,
{
    fn operator(&self, attr: Tangent<Dim>, sdf: &Sdf, p: Dim) -> Dim {
        sdf.evaluate(attr, p).clone().normalize()
    }
}

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Uv> for NormalizeOp
where
    Sdf: FieldFunction<Dim, Uv>,
{
    fn operator(&self, attr: Uv, sdf: &Sdf, p: Dim) -> Vec2 {
        sdf.evaluate(attr, p)
    }
}

impl<Sdf, Dim> SignedDistanceOperator<Sdf, Dim, Color> for NormalizeOp
where
    Sdf: FieldFunction<Dim, Color>,
{
    fn operator(&self, attr: Color, sdf: &Sdf, p: Dim) -> Vec4 {
        sdf.evaluate(attr, p)
    }
}

pub type Normalize<Sdf> = Operator<NormalizeOp, Sdf>;
