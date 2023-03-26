use crate::prelude::{Distance, FieldFunction, Normal, Uv};

use super::FieldOperator;

/// Override the normals of an SDF with the normals of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SdfNormals;

impl<SdfA, SdfB, Pos> FieldOperator<(SdfA, SdfB), Pos, Distance> for SdfNormals
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

impl<SdfA, SdfB, Pos> FieldOperator<(SdfA, SdfB), Pos, Normal<Pos>> for SdfNormals
where
    SdfB: FieldFunction<Pos, Normal<Pos>>,
{
    fn operator(
        &self,
        attr: Normal<Pos>,
        (_, sdf): &(SdfA, SdfB),
        p: Pos,
    ) -> <Normal<Pos> as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, Pos> FieldOperator<(SdfA, SdfB), Pos, Uv> for SdfNormals
where
    SdfA: FieldFunction<Pos, Uv>,
{
    fn operator(
        &self,
        attr: Uv,
        (sdf, _): &(SdfA, SdfB),
        p: Pos,
    ) -> <Uv as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}
