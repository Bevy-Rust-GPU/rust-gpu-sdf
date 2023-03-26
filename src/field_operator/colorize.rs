use rust_gpu_bridge::glam::{Vec3, Vec4};
use type_fields::Field;

use crate::prelude::{Color, Distance, FieldFunction, Normal, Uv};

use super::{FieldOperator, Operator};

/// Apply triplanar UV mapping to the provided SDF
#[derive(Copy, Clone, PartialEq, Field)]
pub struct ColorizeOp {
    pub color: Vec4,
}

impl Default for ColorizeOp {
    fn default() -> Self {
        ColorizeOp { color: Vec4::ONE }
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Distance> for ColorizeOp
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

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Normal<Dim>> for ColorizeOp
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

impl<Sdf> FieldOperator<Sdf, Vec3, Uv> for ColorizeOp
where
    Sdf: FieldFunction<Vec3, Uv>,
{
    fn operator(&self, attr: Uv, sdf: &Sdf, p: Vec3) -> <Uv as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Color> for ColorizeOp {
    fn operator(&self, _: Color, _: &Sdf, _: Vec3) -> <Color as crate::prelude::Attribute>::Type {
        self.color
    }
}

pub type Colorize<Sdf> = Operator<ColorizeOp, Sdf>;

impl<Sdf> Colorize<Sdf> {
    pub fn color(&mut self) -> &mut Vec4 {
        self.op().color()
    }
}
