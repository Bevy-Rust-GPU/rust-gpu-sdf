use rust_gpu_bridge::glam::{Vec2, Vec2Swizzles, Vec3};
use type_fields::Field;

use crate::prelude::{Color, Distance, FieldFunction, Normal, Normalize, Tangent, Uv};

use super::{FieldOperator, Operator};

/// Calculate a 3D gradient given a 2D UV
#[derive(Copy, Clone, PartialEq, Field)]
pub struct UvGradientOp {
    pub axis: Vec2,
    pub epsilon: f32,
}

impl Default for UvGradientOp {
    fn default() -> Self {
        UvGradientOp {
            axis: Vec2::X,
            epsilon: f32::EPSILON,
        }
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Distance> for UvGradientOp
where
    Sdf: FieldFunction<Dim, Distance>,
{
    fn operator(
        &self,
        attr: Distance,
        sdf: &Sdf,
        p: Dim,
    ) -> <Distance as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Normal<Dim>> for UvGradientOp
where
    Sdf: FieldFunction<Dim, Normal<Dim>>,
{
    fn operator(
        &self,
        attr: Normal<Dim>,
        sdf: &Sdf,
        p: Dim,
    ) -> <Normal<Dim> as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Uv> for UvGradientOp
where
    Sdf: FieldFunction<Vec3, Uv>,
{
    fn operator(
        &self,
        attr: Uv,
        sdf: &Sdf,
        p: Vec3,
    ) -> <Uv as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Tangent<Vec3>> for UvGradientOp
where
    Sdf: FieldFunction<Vec3, Uv>,
{
    fn operator(
        &self,
        _: Tangent<Vec3>,
        sdf: &Sdf,
        p: Vec3,
    ) -> <Tangent<Vec3> as crate::prelude::Attribute>::Type {
        let k = Vec2::new(1.0, -1.0);
        k.xyy() * sdf.evaluate(Uv, p + k.xyy() * self.epsilon).dot(self.axis)
            + k.yyx() * sdf.evaluate(Uv, p + k.yyx() * self.epsilon).dot(self.axis)
            + k.yxy() * sdf.evaluate(Uv, p + k.yxy() * self.epsilon).dot(self.axis)
            + k.xxx() * sdf.evaluate(Uv, p + k.xxx() * self.epsilon).dot(self.axis)
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Color> for UvGradientOp
where
    Sdf: FieldFunction<Vec3, Color>,
{
    fn operator(
        &self,
        attr: Color,
        sdf: &Sdf,
        p: Vec3,
    ) -> <Color as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}

pub type UvGradient<Sdf> = Operator<UvGradientOp, Sdf>;

impl<Sdf> UvGradient<Sdf> {
    pub fn axis(&mut self) -> &mut Vec2 {
        self.op().axis()
    }

    pub fn epsilon(&mut self) -> &mut f32 {
        self.op().epsilon()
    }
}

pub type UvTangent<Sdf> = Normalize<UvGradient<Sdf>>;

impl<Sdf> UvTangent<Sdf> {
    pub fn sdf(&mut self) -> &mut Sdf {
        self.target().target()
    }

    pub fn axis(&mut self) -> &mut Vec2 {
        self.target().axis()
    }

    pub fn epsilon(&mut self) -> &mut f32 {
        self.target().epsilon()
    }
}
