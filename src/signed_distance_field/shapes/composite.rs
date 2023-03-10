//! Shapes composed from other shapes.

use rust_gpu_bridge::prelude::{Vec2, Vec3};

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
pub type Line<Dim> = ExtrudeDist<Point, Dim>;

/// A 2D circle.
pub type Circle = Round<Point, Vec2>;

/// A 3D sphere.
pub type Sphere = Round<Point, Vec3>;

/// A capsule.
pub type Capsule<Dim> = Operator<Line<Dim>, RoundOp, Dim>;

/// A cube.
pub type Cube<Dim> = Elongate<Point, Exact, Dim>;
