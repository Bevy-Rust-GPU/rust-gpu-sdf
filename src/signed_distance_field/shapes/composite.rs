//! Shapes composed from other shapes.

use rust_gpu_bridge::prelude::Vec2;
use type_fields::field::Field;

use crate::{
    operators::{elongate::Elongate, isosurface::Isosurface, stretch::StretchDist},
    prelude::{
        axial_reflect::{AxialReflect, AXIS_X, AXIS_XY},
        sided::Sided,
        Reflect, Translate,
    },
    signed_distance_field::{adapters::sweep::Sweep, metrics::euclidean::EuclideanMetric},
    D2, D3,
};

/// An infinitely small point.
/// Not very useful on its own; primarily used for composition.
pub type Point = EuclideanMetric;

/// An infinitely thin line.
/// Not very useful on its own; primarily used for composition.
pub type Line<Dim> = StretchDist<Dim, Point>;

/// A ball.
pub type Ball = Isosurface<Point>;

impl Ball {
    pub fn radius(&mut self) -> &mut f32 {
        &mut self.op.delta
    }
}

/// A 2D circle.
pub type Circle = Ball;

/// A 3D sphere.
pub type Sphere = Ball;

/// A capsule.
pub type Capsule<Dim> = Isosurface<Line<Dim>>;

/// A box.
pub type Box<Dim> = Elongate<Dim, Point>;

/// A 2D square.
pub type Square = Box<D2>;

pub type NgonMirror<Sdf> = Reflect<Vec2, Sdf>;

impl<Sdf> NgonMirror<Sdf> {
    pub fn new(sin: f32, cos: f32) -> Self
    where
        Sdf: Default + 'static,
    {
        <NgonMirror<Sdf> as Default>::default()
            .with(Self::sin, sin)
            .with(Self::cos, cos)
    }

    pub fn sdf(&mut self) -> &mut Sdf {
        &mut self.target
    }

    pub fn sin(&mut self) -> &mut f32 {
        &mut self.op.axis.y
    }

    pub fn cos(&mut self) -> &mut f32 {
        &mut self.op.axis.x
    }
}

pub type Triangle =
    AxialReflect<NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>, AXIS_X>;

impl Triangle {
    pub fn triangle() -> Self {
        let angle = core::f32::consts::PI / 3 as f32;
        let sin = angle.sin();
        let cos = angle.cos();
        let tan = angle.tan();
        <Self as Default>::default()
            .with(Self::target, NgonMirror::new(sin, -cos))
            .with(Self::tan, tan)
            .with(Self::radius, 1.0)
    }

    pub fn tan(&mut self) -> &mut f32 {
        self.target().sdf().dist()
    }

    pub fn radius(&mut self) -> &mut f32 {
        &mut self.target().sdf().target().translation().y
    }
}

pub type Quadrilateral =
    AxialReflect<NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>, AXIS_XY>;

impl Quadrilateral {
    pub fn quadrilateral() -> Self {
        let angle = core::f32::consts::PI / 4 as f32;
        let sin = angle.sin();
        let cos = angle.cos();
        let tan = angle.tan();
        <Self as Default>::default()
            .with(Self::target, NgonMirror::new(sin, -cos))
            .with(Self::tan, tan)
            .with(Self::radius, 1.0)
    }

    pub fn tan(&mut self) -> &mut f32 {
        self.target().sdf().dist()
    }

    pub fn radius(&mut self) -> &mut f32 {
        &mut self.target().sdf().target().translation().y
    }
}

pub type Pentagon = AxialReflect<
    NgonMirror<NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>>,
    AXIS_X,
>;

impl Pentagon {
    pub fn pentagon() -> Self {
        let angle = core::f32::consts::PI / 5 as f32;
        let sin = angle.sin();
        let cos = angle.cos();
        let tan = angle.tan();

        <Self as Default>::default()
            .with(Self::target, NgonMirror::new(sin, -cos))
            .with((Self::target, NgonMirror::sdf), NgonMirror::new(sin, cos))
            .with(Self::tan, tan)
            .with(Self::radius, 1.0)
    }

    pub fn tan(&mut self) -> &mut f32 {
        &mut self.target().sdf().sdf().op.dist
    }

    pub fn radius(&mut self) -> &mut f32 {
        &mut self.target().sdf().sdf().target.op.translation.y
    }
}

pub type Hexagon =
    AxialReflect<NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>, AXIS_XY>;

impl Hexagon {
    pub fn hexagon() -> Self {
        let angle = core::f32::consts::PI / 6 as f32;
        let sin = angle.sin();
        let cos = angle.cos();
        let tan = angle.tan();

        <Self as Default>::default()
            .with(Self::target, NgonMirror::new(sin, -cos))
            .with(Self::tan, tan)
            .with(Self::radius, 1.0)
    }
}

pub type Septagon = AxialReflect<
    NgonMirror<NgonMirror<NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>>>,
    AXIS_X,
>;

impl Septagon {
    pub fn septagon() -> Self {
        let angle = core::f32::consts::PI / 7 as f32;
        let sin = angle.sin();
        let cos = angle.cos();
        let tan = angle.tan();

        <Self as Default>::default()
            .with(Self::target, NgonMirror::new(sin, -cos))
            .with((Self::target, NgonMirror::sdf), NgonMirror::new(sin, cos))
            .with(
                (Self::target, NgonMirror::sdf, NgonMirror::sdf),
                NgonMirror::new(sin, -cos),
            )
            .with(Self::tan, tan)
            .with(Self::radius, 1.0)
    }

    pub fn tan(&mut self) -> &mut f32 {
        &mut self.target().sdf().sdf().sdf().op.dist
    }

    pub fn radius(&mut self) -> &mut f32 {
        &mut self.target().sdf().sdf().sdf().target.op.translation.y
    }
}

pub type Octagon = AxialReflect<
    NgonMirror<NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>>,
    AXIS_XY,
>;

impl Octagon {
    pub fn octagon() -> Self {
        let angle = core::f32::consts::PI / 8 as f32;
        let sin = angle.sin();
        let cos = angle.cos();
        let tan = angle.tan();

        <Self as Default>::default()
            .with(Self::target, NgonMirror::new(sin, -cos))
            .with((Self::target, NgonMirror::sdf), NgonMirror::new(sin, cos))
            .with(Self::tan, tan)
            .with(Self::radius, 1.0)
    }

    pub fn tan(&mut self) -> &mut f32 {
        &mut self.target().sdf().sdf().op.dist
    }

    pub fn radius(&mut self) -> &mut f32 {
        &mut self.target().sdf().sdf().target.op.translation.y
    }
}

pub type Nonagon = AxialReflect<
    NgonMirror<
        NgonMirror<NgonMirror<NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>>>,
    >,
    AXIS_X,
>;

impl Nonagon {
    pub fn nonagon() -> Self {
        let angle = core::f32::consts::PI / 9 as f32;
        let sin = angle.sin();
        let cos = angle.cos();
        let tan = angle.tan();

        <Self as Default>::default()
            .with(Self::target, NgonMirror::new(sin, -cos))
            .with((Self::target, NgonMirror::sdf), NgonMirror::new(sin, cos))
            .with(
                (Self::target, NgonMirror::sdf, NgonMirror::sdf),
                NgonMirror::new(sin, -cos),
            )
            .with(
                (
                    Self::target,
                    NgonMirror::sdf,
                    NgonMirror::sdf,
                    NgonMirror::sdf,
                ),
                NgonMirror::new(sin, cos),
            )
            .with(Self::tan, tan)
            .with(Self::radius, 1.0)
    }

    pub fn tan(&mut self) -> &mut f32 {
        &mut self.target().sdf().sdf().sdf().sdf().op.dist
    }

    pub fn radius(&mut self) -> &mut f32 {
        &mut self
            .target()
            .sdf()
            .sdf()
            .sdf()
            .sdf()
            .target
            .op
            .translation
            .y
    }
}

pub type Decagon = AxialReflect<
    NgonMirror<NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>>,
    AXIS_XY,
>;

impl Decagon {
    pub fn decagon() -> Self {
        let angle = core::f32::consts::PI / 9 as f32;
        let sin = angle.sin();
        let cos = angle.cos();
        let tan = angle.tan();

        <Self as Default>::default()
            .with(Self::target, NgonMirror::new(sin, -cos))
            .with((Self::target, NgonMirror::sdf), NgonMirror::new(sin, cos))
            .with(Self::tan, tan)
            .with(Self::radius, 1.0)
    }
}

/// A 3D cube.
pub type Cube = Box<D3>;

/// A 3D torus.
pub type Torus = Sweep<Circle, Circle>;

#[cfg(test)]
pub mod tests {
    use type_fields::field::Field;

    use super::{Circle, Sphere};

    #[test]
    fn test_circle() {
        Circle::default().with(Circle::radius, f32::default());
    }

    #[test]
    fn test_sphere() {
        Sphere::default().with(Sphere::radius, f32::default());
    }
}
