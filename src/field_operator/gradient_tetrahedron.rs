use rust_gpu_bridge::glam::{Vec2, Vec2Swizzles, Vec3};
use type_fields::Field;

use crate::prelude::{Distance, FieldFunction, Normal, Normalize};

use super::{FieldOperator, Operator};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct GradientTetrahedronOp {
    pub epsilon: f32,
}

impl Default for GradientTetrahedronOp {
    fn default() -> Self {
        GradientTetrahedronOp {
            epsilon: f32::EPSILON,
        }
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Distance> for GradientTetrahedronOp
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

impl<Sdf> FieldOperator<Sdf, Vec2, Normal<Vec2>> for GradientTetrahedronOp
where
    Sdf: FieldFunction<Vec2, Distance>,
{
    fn operator(
        &self,
        _: Normal<Vec2>,
        sdf: &Sdf,
        p: Vec2,
    ) -> <Normal<Vec2> as crate::prelude::Attribute>::Type {
        let k = Vec2::new(1.0, -1.0);
        k.xy() * sdf.evaluate(Distance, p + k.xy() * self.epsilon)
            + k.yy() * sdf.evaluate(Distance, p + k.yy() * self.epsilon)
            + k.yx() * sdf.evaluate(Distance, p + k.yx() * self.epsilon)
            + k.xx() * sdf.evaluate(Distance, p + k.xx() * self.epsilon)
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Normal<Vec3>> for GradientTetrahedronOp
where
    Sdf: FieldFunction<Vec3, Distance>,
{
    fn operator(
        &self,
        _: Normal<Vec3>,
        sdf: &Sdf,
        p: Vec3,
    ) -> <Normal<Vec3> as crate::prelude::Attribute>::Type {
        let k = Vec2::new(1.0, -1.0);
        k.xyy() * sdf.evaluate(Distance, p + k.xyy() * self.epsilon)
            + k.yyx() * sdf.evaluate(Distance, p + k.yyx() * self.epsilon)
            + k.yxy() * sdf.evaluate(Distance, p + k.yxy() * self.epsilon)
            + k.xxx() * sdf.evaluate(Distance, p + k.xxx() * self.epsilon)
    }
}

pub type GradientTetrahedron<Sdf> = Operator<GradientTetrahedronOp, Sdf>;

impl<Sdf> GradientTetrahedron<Sdf> {
    pub fn epsilon(&mut self) -> &mut f32 {
        self.op().epsilon()
    }
}

pub type NormalTetrahedron<Sdf> = Normalize<GradientTetrahedron<Sdf>>;

impl<Sdf> NormalTetrahedron<Sdf> {
    pub fn sdf(&mut self) -> &mut Sdf {
        self.target().target()
    }

    pub fn epsilon(&mut self) -> &mut f32 {
        self.target().epsilon()
    }
}
