//! Utility type for testing the bound error term of a distance function

use core::ops::RangeInclusive;

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};

use crate::prelude::{
    default, AttrBoundError, AttrDistance, AttrNormal, BoundError, Field,
    FieldAttribute, SupportFunction,
};

/// Asserts that the provided distance function is a field rather than a bound
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoundTester<Sdf> {
    pub sdf: Sdf,
    pub samples: RangeInclusive<isize>,
    pub step: f32,
    pub epsilon: f32,
}

impl<Sdf> Default for BoundTester<Sdf>
where
    Sdf: Default,
{
    fn default() -> Self {
        BoundTester {
            sdf: default(),
            samples: -20..=20,
            step: 10.0 / 20.0,
            epsilon: 0.00001,
        }
    }
}

impl<Sdf> BoundTester<Sdf>
where
    Sdf: Field<AttrDistance<Vec2>> + Field<AttrNormal<Vec2>> + Default + Clone + 'static,
{
    pub fn is_field_2d(self) -> bool {
        !self.is_bound_2d()
    }

    pub fn is_bound_2d(self) -> bool {
        // Iterate over a regular grid
        for x in self.samples.clone() {
            for y in self.samples.clone() {
                // Compose sample coordinate
                let pos = Vec2::new(x as f32, y as f32) * self.step;

                // Calculate error term
                let error_term = BoundError {
                    target: SupportFunction {
                        target: self.sdf.clone(),
                        ..default()
                    },
                    ..Default::default()
                }
                .attribute::<AttrBoundError<Vec2>>(&pos.into());

                // Skip samples with no valid support function
                if error_term.support.normal == Vec2::ZERO {
                    continue;
                }

                assert!(
                    error_term.error.abs() <= self.epsilon,
                    "Encountered error {:?} at point {:}, {:} with {:?} and normal {:}, {}",
                    error_term.error,
                    pos.x,
                    pos.y,
                    error_term.support.distance,
                    error_term.support.normal.x,
                    error_term.support.normal.y
                );
            }
        }

        false
    }
}

impl<Sdf> BoundTester<Sdf>
where
    Sdf: Field<AttrDistance<Vec3>> + Field<AttrNormal<Vec3>> + Default + Clone + 'static,
{
    pub fn is_field_3d(self) -> bool {
        !self.is_bound_3d()
    }

    pub fn is_bound_3d(self) -> bool {
        // Iterate over a regular grid
        for x in self.samples.clone() {
            for y in self.samples.clone() {
                for z in self.samples.clone() {
                    // Compose sample coordinate
                    let pos = Vec3::new(x as f32, y as f32, z as f32) * self.step;

                    // Calculate error term
                    let error_term = BoundError {
                        target: SupportFunction {
                            target: self.sdf.clone(),
                            ..default()
                        },
                        ..Default::default()
                    }
                    .attribute::<AttrBoundError<Vec3>>(&pos.into());

                    // Skip samples with no valid support function
                    if error_term.support.normal == Vec3::ZERO {
                        continue;
                    }

                    assert!(
                    error_term.error.abs() <= self.epsilon,
                    "Encountered error {:?} at point {:}, {:}, {:} with {:?} and normal {:}, {:}, {:}",
                    error_term.error,
                    pos.x,
                    pos.y,
                    pos.z,
                    error_term.support.distance,
                    error_term.support.normal.x,
                    error_term.support.normal.y,
                    error_term.support.normal.z
                );
                }
            }
        }

        false
    }
}
