//! Override the UVs of an SDF with the UVs of another SDF

use type_fields::Field;

use crate::prelude::{Distance, FieldFunction, Normal, Uv};

use super::{FieldOperator, Operator};

/// Override the UVs of an SDF with the UVs of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct SdfUvOp;

impl<SdfA, SdfB, Pos> FieldOperator<(SdfA, SdfB), Pos, Distance> for SdfUvOp
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

impl<SdfA, SdfB, In> FieldOperator<(SdfA, SdfB), In, Normal<In>> for SdfUvOp
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

impl<SdfA, SdfB, In> FieldOperator<(SdfA, SdfB), In, Uv> for SdfUvOp
where
    SdfB: FieldFunction<In, Uv>,
{
    fn operator(
        &self,
        attr: Uv,
        (_, sdf): &(SdfA, SdfB),
        p: In,
    ) -> <Uv as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}

pub type SdfUv<SdfA, SdfB> = Operator<SdfUvOp, (SdfA, SdfB)>;
