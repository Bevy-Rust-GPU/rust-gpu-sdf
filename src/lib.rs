#![no_std]

pub mod markers;
pub mod operators;
pub mod signed_distance_field;

pub mod parameters {
    //! Proof-of-concept configuration API for compositional types

    // Doesn't seem sufficient; not suitable for compositions that have
    // more than one nested instance of the same type.
    
    // Will probably need to look into functional lenses.

    pub trait SignedDistanceParameter<Parameter> {
        type Type;

        fn with(self, parameter: Parameter, value: Self::Type) -> Self;
    }

    #[derive(Default)]
    struct Wrapper<Sdf> {
        sdf: Sdf,
    }

    impl<Sdf, Parameter> SignedDistanceParameter<Parameter> for Wrapper<Sdf>
    where
        Sdf: SignedDistanceParameter<Parameter>,
    {
        type Type = Sdf::Type;

        fn with(mut self, parameter: Parameter, value: Self::Type) -> Self {
            self.sdf = self.sdf.with(parameter, value);
            self
        }
    }

    pub struct Radius;

    #[derive(Default)]
    struct Leaf {
        radius: f32,
    }

    impl SignedDistanceParameter<Radius> for Leaf {
        type Type = f32;

        fn with(mut self, _: Radius, value: Self::Type) -> Self {
            self.radius = value;
            self
        }
    }

    #[cfg(test)]
    pub mod tests {
        use super::{Leaf, Radius, SignedDistanceParameter, Wrapper};

        #[test]
        fn test_parameters() {
            let comp = Wrapper::<Leaf>::default().with(Radius, 1.0);
        }
    }
}

#[cfg(test)]
pub mod tests;

use rust_gpu_bridge::prelude::{Vec2, Vec3};

pub type D2 = Vec2;
pub type D3 = Vec3;

/// Free-standing [`Default::default()`] invocation
pub fn default<T: Default>() -> T {
    Default::default()
}
