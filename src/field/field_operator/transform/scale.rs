//! Uniformly scale a distance field.

use core::ops::{Div, Mul};

use type_fields::macros::Field;

use crate::prelude::{Attribute, Field, FieldOperator, Operator};

/// Uniformly scale a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct ScaleOp {
    pub scale: f32,
}

impl<Sdf, Attr> FieldOperator<Sdf, Attr> for ScaleOp
where
    Attr: Attribute,
    Sdf: Field<Attr>,
    Attr::Input: Clone + Div<f32, Output = Attr::Input>,
    Attr::Output: Mul<f32, Output = Attr::Output>,
{
    fn operator(&self, sdf: &Sdf, input: &Attr::Input) -> Attr::Output {
        sdf.field(&(input.clone() / self.scale)).mul(self.scale)
    }
}

/// Uniformly scale a distance field.
pub type Scale<Sdf> = Operator<ScaleOp, Sdf>;

impl<Sdf> Scale<Sdf> {
    pub fn scale(&mut self) -> &mut f32 {
        &mut self.op.scale
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use type_fields::field::Field;

    use crate::{
        prelude::{Cube, Point, Scale},
        test_op_attrs,
    };

    #[test]
    fn test_scale() {
        Scale::<Cube>::default().with(Scale::scale, f32::default());
    }

    test_op_attrs!(Scale::<Point>);
}
