//! Operators composed from other operators.

use crate::prelude::{Hollow, Isosurface};

/// Converts a solid shape into a hollow one with the given surface depth.
pub type Onion<Sdf> = Isosurface<Hollow<Sdf>>;

impl<Sdf> Onion<Sdf> {
    pub fn radius(&mut self) -> &mut f32 {
        self.op().delta()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use type_fields::field::Field;

    use crate::{
        prelude::{Circle, Onion},
        test_op_attrs,
    };

    #[test]
    fn test_onion() {
        Onion::<Circle>::default().with(Onion::radius, f32::default());
    }

    test_op_attrs!(Onion::<Circle>);
}
