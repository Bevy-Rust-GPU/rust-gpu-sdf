//! Utility type for testing the bound error term of a distance function

use core::{marker::PhantomData, ops::RangeInclusive};

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};

use crate::prelude::{
    default, BoundError, Distance, ErrorTerm, FieldAttribute, FieldFunction, Normal, Support,
    SupportFunction,
};

/// Asserts that the provided distance function is a field rather than a bound
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoundTester<Dim, Sdf> {
    pub sdf: Sdf,
    pub samples: RangeInclusive<isize>,
    pub step: f32,
    pub epsilon: f32,
    pub _phantom: PhantomData<Dim>,
}

impl<Dim, Sdf> Default for BoundTester<Dim, Sdf>
where
    Sdf: Default,
{
    fn default() -> Self {
        BoundTester {
            sdf: default(),
            samples: -20..=20,
            step: 10.0 / 20.0,
            epsilon: 0.00001,
            _phantom: default(),
        }
    }
}

impl<Sdf> BoundTester<Vec2, Sdf>
where
    Sdf: FieldFunction<Vec2, Distance>
        + FieldFunction<Vec2, Normal<Vec2>>
        + Default
        + Clone
        + 'static,
{
    pub fn is_field(self) -> bool {
        !self.is_bound()
    }

    pub fn is_bound(self) -> bool {
        // Iterate over a regular grid
        for x in self.samples.clone() {
            for y in self.samples.clone() {
                // Compose sample coordinate
                let pos = Vec2::new(x as f32, y as f32) * self.step;

                // Calculate support
                let error_term = BoundError {
                    target: SupportFunction {
                        target: self.sdf.clone(),
                        ..default()
                    },
                    ..Default::default()
                }
                .attribute::<ErrorTerm<Vec2>>(pos);

                assert!(
                    error_term.error.abs() <= self.epsilon,
                    "Encountered error term {:?} at point {:}, {:} with distance {:} and normal {:}, {}",
                    pos.x,
                    pos.y,
                    error_term.error,
                    error_term.support.distance,
                    error_term.support.normal.x,
                    error_term.support.normal.y
                );
            }
        }

        false
    }
}

impl<Sdf> BoundTester<Vec3, Sdf>
where
    Sdf: FieldFunction<Vec3, Distance>
        + FieldFunction<Vec3, Normal<Vec3>>
        + Default
        + Clone
        + 'static,
{
    pub fn is_field(self) -> bool {
        !self.is_bound()
    }

    pub fn is_bound(self) -> bool {
        // Iterate over a regular grid
        for x in self.samples.clone() {
            for y in self.samples.clone() {
                for z in self.samples.clone() {
                    // Compose sample coordinate
                    let pos = Vec3::new(x as f32, y as f32, z as f32) * self.step;

                    // Calculate support
                    let support = SupportFunction {
                        target: self.sdf.clone(),
                        ..Default::default()
                    }
                    .field(Support::<Vec3>::default(), pos);

                    // Skip samples with no valid support function
                    if support.normal == Vec3::ZERO {
                        continue;
                    }

                    // Evaluate distance at surface vector, asserting that it is near zero
                    let r = self.sdf.field(Distance, pos + support.support_vector());
                    assert!(
                    r.abs() <= self.epsilon,
                    "Encountered reciprocal distance {r:} at point {:}, {:}, {:} with distance {:} and normal {:}, {:}, {:}",
                    pos.x,
                    pos.y,
                    pos.z,
                    support.distance,
                    support.normal.x,
                    support.normal.y,
                    support.normal.z
                );
                }
            }
        }

        false
    }
}
