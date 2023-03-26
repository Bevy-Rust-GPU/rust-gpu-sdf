//! Override the colors of an SDF with the colors of another SDF

use type_fields::Field;

use crate::prelude::{Color, Distance, FieldFunction, Normal, Uv};

use super::{FieldOperator, Operator};

/// Override the colors of an SDF with the colors of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct SdfColorOp;

impl<SdfA, SdfB, Pos> FieldOperator<(SdfA, SdfB), Pos, Distance> for SdfColorOp
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

impl<SdfA, SdfB, Pos> FieldOperator<(SdfA, SdfB), Pos, Normal<Pos>> for SdfColorOp
where
    SdfA: FieldFunction<Pos, Normal<Pos>>,
{
    fn operator(
        &self,
        attr: Normal<Pos>,
        (sdf, _): &(SdfA, SdfB),
        p: Pos,
    ) -> <Normal<Pos> as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, In> FieldOperator<(SdfA, SdfB), In, Uv> for SdfColorOp
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

impl<SdfA, SdfB, In> FieldOperator<(SdfA, SdfB), In, Color> for SdfColorOp
where
    SdfB: FieldFunction<In, Color>,
{
    fn operator(
        &self,
        attr: Color,
        (_, sdf): &(SdfA, SdfB),
        p: In,
    ) -> <Color as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}

pub type SdfColor<SdfA, SdfB> = Operator<SdfColorOp, (SdfA, SdfB)>;
