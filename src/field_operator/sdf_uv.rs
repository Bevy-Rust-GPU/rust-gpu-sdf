//! Override the UVs of an SDF with the UVs of another SDF

use type_fields::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{Color, Distance, Normal, Tangent, Uv},
};

use super::{FieldOperator, Operator};

/// Override the UVs of an SDF with the UVs of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct SdfUvOp;

impl_passthrough_op_2!(SdfUvOp, <Dim>, Distance, 0);
impl_passthrough_op_2!(SdfUvOp, <Dim>, Normal<Dim>, 0);
impl_passthrough_op_2!(SdfUvOp, <Dim>, Tangent<Dim>, 0);
impl_passthrough_op_2!(SdfUvOp, <Dim>, Uv, 1);
impl_passthrough_op_2!(SdfUvOp, <Dim>, Color, 0);

pub type SdfUv<SdfA, SdfB> = Operator<SdfUvOp, (SdfA, SdfB)>;
