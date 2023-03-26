use rust_gpu_bridge::glam::Vec3;
use type_fields::Field;

use crate::prelude::{Color, Distance, FieldFunction, Normal, Tangent, Uv};

use super::FieldOperator;

/// Override the tangents of an SDF with the tangents of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct SdfTangentOp;

impl<SdfA, SdfB, Pos> FieldOperator<(SdfA, SdfB), Pos, Distance> for SdfTangentOp
where
    SdfA: FieldFunction<Pos, Distance>,
{
    fn operator(
        &self,
        attr: Distance,
        (sdf, _): &(SdfA, SdfB),
        p: Pos,
    ) -> <Distance as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, In> FieldOperator<(SdfA, SdfB), In, Normal<In>> for SdfTangentOp
where
    SdfA: FieldFunction<In, Normal<In>>,
{
    fn operator(
        &self,
        attr: Normal<In>,
        (sdf, _): &(SdfA, SdfB),
        p: In,
    ) -> <Normal<In> as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, In> FieldOperator<(SdfA, SdfB), In, Uv> for SdfTangentOp
where
    SdfA: FieldFunction<In, Uv>,
{
    fn operator(
        &self,
        attr: Uv,
        (sdf, _): &(SdfA, SdfB),
        p: In,
    ) -> <Uv as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, Pos> FieldOperator<(SdfA, SdfB), Pos, Tangent<Vec3>> for SdfTangentOp
where
    SdfB: FieldFunction<Pos, Tangent<Vec3>>,
{
    fn operator(
        &self,
        attr: Tangent<Vec3>,
        (_, sdf): &(SdfA, SdfB),
        p: Pos,
    ) -> <Tangent<Vec3> as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, In> FieldOperator<(SdfA, SdfB), In, Color> for SdfTangentOp
where
    SdfA: FieldFunction<In, Color>,
{
    fn operator(
        &self,
        attr: Color,
        (sdf, _): &(SdfA, SdfB),
        p: In,
    ) -> <Color as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}

