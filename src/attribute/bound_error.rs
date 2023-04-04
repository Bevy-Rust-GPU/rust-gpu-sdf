//! Error term quantifying the bound-ness of a distance function.
//!
//! A distance function can be considered a correct distance field
//! if its derivative is uniformly 1.
//! If this does not hold, it is instead considered a distance bound.
//!
//! In practical terms, this equates to any stretching, squashing,
//! incorrectly-sharp edges, or other discontinuities in an evaluated field.
//!
//! This can only be tested for, but the accuracy of the test is determined
//! by the accuracy of the field's derivative function.
//!
//! This creates issues when testing fields whose derivatives
//! are calculated using local differencing, as the process
//! innately smooths off discontinuities relative to its epsilon factor.
//!
//! To avoid this, we combine the gradient at a given point in the field
//! with a distance evaluation to produce a support vector;
//! i.e. the vector from the evaluated position
//! to the nearest point on the implicit surface.
//!
//! In a correct distance field, summing the evaluated position
//! and support vector will result in a new position whose
//! evaluated distance is almost zero w.r.t. floating-point precision.
//!
//! This is still subject to its own error term relative to the accuracy of the
//! gradient function, but is more robust than the derivative approach,
//! and able to catch more common bound cases.

use core::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use crate::{
    impl_passthrough_op_1,
    prelude::{
        items::position::Position, AttrColor, AttrDistance, AttrNormal, AttrSupport, AttrTangent,
        AttrUv, Attribute, Distance, Field, FieldOperator, Operator, Support, SupportFunction,
    },
};

/// Bound error term
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub struct ErrorTerm<Dim> {
    pub support: Support<Dim>,
    pub error: Distance,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttrBoundError<Dim> {
    _phantom: PhantomData<Dim>,
}

impl<Dim> Attribute for AttrBoundError<Dim>
where
    Dim: Default,
{
    type Input = Position<Dim>;
    type Output = ErrorTerm<Dim>;
}

/// Bound error wrapper operator
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BoundErrorOp;

impl<Sdf, Input> FieldOperator<Sdf, AttrBoundError<Input>> for BoundErrorOp
where
    Sdf: Field<AttrDistance<Input>> + Field<AttrSupport<Input>>,
    Input: Default + Clone + Add<Input, Output = Input> + Mul<f32, Output = Input>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: &Position<Input>,
    ) -> <AttrBoundError<Input> as Attribute>::Output {
        let mut out = ErrorTerm::default();

        let support = Field::<AttrSupport<Input>>::field(sdf, p);
        let sv = support.support_vector();
        out.support = support;
        out.error = Field::<AttrDistance<Input>>::field(sdf, &((*p).clone() + sv));
        out
    }
}

impl_passthrough_op_1!(BoundErrorOp, AttrDistance<Dim>, Dim);
impl_passthrough_op_1!(BoundErrorOp, AttrNormal<Dim>, Dim);
impl_passthrough_op_1!(BoundErrorOp, AttrTangent<Dim>, Dim);
impl_passthrough_op_1!(BoundErrorOp, AttrUv<Dim>, Dim);
impl_passthrough_op_1!(BoundErrorOp, AttrColor<Dim>, Dim);

/// Bound error wrapper
pub type BoundError<Sdf> = Operator<BoundErrorOp, SupportFunction<Sdf>>;
