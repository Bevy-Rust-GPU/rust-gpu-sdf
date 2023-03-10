//! Shapes composed from other shapes.

use rust_gpu_bridge::prelude::Vec3;

use crate::{
    markers::Exact,
    operators::{
        elongate::Elongate,
        extrude::ExtrudeDist,
        round::{Round, RoundOp},
        Operator,
    },
    signed_distance_field::metrics::euclidean::EuclideanMetric,
};

/// An infinitely small point.
/// Not very useful on its own; primarily used for composition.
pub type Point = EuclideanMetric;

/// An infinitely thin line.
/// Not very useful on its own; primarily used for composition.
pub type Line = ExtrudeDist<Point>;

/// A sphere.
pub type Sphere = Round<Point>;

/// A capsule.
pub type Capsule = Operator<Line, RoundOp, Vec3>;

/// A cube.
pub type Cube = Elongate<Point, Exact>;

