//! Shapes composed from other shapes.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Cos, Sin, Sqrt, Tan,
};

use crate::prelude::{
    raytrace::RayIntersection, AxialReflect, Elongate, EuclideanMetric, Isosurface, Reflect, Sided,
    StretchDist, Sweep, Translate, AXIS_X, AXIS_XY, D2, D3,
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
        self.op().delta()
    }
}

/// A 2D circle.
pub type Circle = Ball;

/// A 3D sphere.
pub type Sphere = Ball;

impl RayIntersection for Sphere {
    fn intersect(&self, eye: Vec3, dir: Vec3) -> Option<f32> {
        let b = eye.dot(dir);
        let r = self.op.delta;
        let c = eye.dot(eye) - r * r;

        // Exit if r’s origin outside s (c > 0) and r pointing away from s (b > 0)
        if c > 0.0 && b > 0.0 {
            return None;
        }
        let discr = b * b - c;

        // A negative discriminant corresponds to ray missing sphere
        if discr < 0.0 {
            return None;
        }

        // Ray now found to intersect sphere, compute smallest t value of intersection
        let mut t = -b - discr.sqrt();

        // If t is negative, ray started inside sphere so clamp t to zero
        if t < 0.0 {
            t = 0.0;
        }

        Some(t)
    }
}

/// A capsule.
pub type Capsule<Dim> = Isosurface<Line<Dim>>;

/// A box.
pub type Box<Dim> = Elongate<Dim, Point>;

/// A 2D square.
pub type Square = Box<D2>;

/// A 3D cube.
pub type Cube = Box<D3>;

/// A 3D torus.
pub type Torus = Sweep<Circle, Circle>;

impl Torus {
    pub fn torus() -> Self {
        use type_fields::field::Field;

        <Self as Default>::default()
            .with((Torus::core, Circle::radius), 0.75)
            .with((Torus::shell, Circle::radius), 0.25)
    }
}

pub type NgonMirror<Sdf> = Reflect<Vec2, Sdf>;

impl<Sdf> NgonMirror<Sdf> {
    pub fn new(sin: f32, cos: f32) -> Self
    where
        Sdf: Default + 'static,
    {
        use type_fields::field::Field;
        let mut sdf = <NgonMirror<Sdf> as Default>::default()
            .with(Self::sin, sin)
            .with(Self::cos, cos);
        *sdf.axis() = sdf.axis().normalize();
        sdf
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
    AxialReflect<AXIS_X, NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>>;

impl Triangle {
    pub fn triangle() -> Self {
        use type_fields::field::Field;
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
    AxialReflect<AXIS_XY, NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>>;

impl Quadrilateral {
    pub fn quadrilateral() -> Self {
        use type_fields::field::Field;
        let angle = core::f32::consts::PI / 4.0;
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
    AXIS_X,
    NgonMirror<NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>>,
>;

impl Pentagon {
    pub fn pentagon() -> Self {
        use type_fields::field::Field;
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
    AxialReflect<AXIS_XY, NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>>;

impl Hexagon {
    pub fn hexagon() -> Self {
        use type_fields::field::Field;
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
    AXIS_X,
    NgonMirror<NgonMirror<NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>>>,
>;

impl Septagon {
    pub fn septagon() -> Self {
        use type_fields::field::Field;
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
    AXIS_XY,
    NgonMirror<NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>>,
>;

impl Octagon {
    pub fn octagon() -> Self {
        use type_fields::field::Field;
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
    AXIS_X,
    NgonMirror<
        NgonMirror<NgonMirror<NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>>>,
    >,
>;

impl Nonagon {
    pub fn nonagon() -> Self {
        use type_fields::field::Field;
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
    AXIS_XY,
    NgonMirror<NgonMirror<NgonMirror<StretchDist<Vec2, Translate<Vec2, Sided<Vec2, Point>>>>>>,
>;

impl Decagon {
    pub fn decagon() -> Self {
        use type_fields::field::Field;
        let angle = core::f32::consts::PI / 10 as f32;
        let sin = angle.sin();
        let cos = angle.cos();
        let tan = angle.tan();

        <Self as Default>::default()
            .with(Self::target, NgonMirror::new(sin, -cos))
            .with((Self::target, NgonMirror::sdf), NgonMirror::new(sin, cos))
            .with(
                (Self::target, NgonMirror::sdf, NgonMirror::sdf),
                NgonMirror::new(sin, cos),
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

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use rust_gpu_bridge::glam::{Vec2, Vec3};

    use crate::prelude::{
        BoundTester, Capsule, Cube, Decagon, Hexagon, Line, Nonagon, Octagon, Pentagon,
        Quadrilateral, Septagon, Square, Torus, Triangle,
    };

    use super::{Circle, Point, Sphere};

    #[test]
    fn test_point_2d() {
        assert!(BoundTester::<Point>::default().is_field_2d());
    }

    #[test]
    fn test_line_2d() {
        assert!(BoundTester::<Line<Vec2>>::default().is_field_2d());
    }

    #[test]
    fn test_point_3d() {
        assert!(BoundTester::<Point>::default().is_field_3d());
    }

    #[test]
    fn test_line_3d() {
        assert!(BoundTester::<Line<Vec3>>::default().is_field_3d());
    }

    #[test]
    fn test_circle() {
        assert!(BoundTester::<Circle>::default().is_field_3d());
    }

    #[test]
    fn test_sphere() {
        assert!(BoundTester::<Sphere>::default().is_field_3d());
    }

    #[test]
    fn test_capsule_2d() {
        assert!(BoundTester::<Capsule<Vec2>>::default().is_field_2d());
    }

    #[test]
    fn test_capsule_3d() {
        assert!(BoundTester::<Capsule<Vec3>>::default().is_field_3d());
    }

    #[test]
    fn test_square() {
        assert!(BoundTester::<Square>::default().is_field_2d());
    }

    #[test]
    fn test_cube() {
        assert!(BoundTester::<Cube>::default().is_field_3d());
    }

    #[test]
    fn test_torus() {
        assert!(BoundTester::<Torus>::default().is_field_3d());
    }

    #[test]
    fn test_triangle() {
        assert!(BoundTester::<Triangle>::default().is_field_2d());
    }

    #[test]
    fn test_quadrilateral() {
        assert!(BoundTester::<Quadrilateral>::default().is_field_2d());
    }

    #[test]
    fn test_pentagon() {
        assert!(BoundTester::<Pentagon>::default().is_field_2d());
    }

    #[test]
    fn test_hexagon() {
        assert!(BoundTester::<Hexagon>::default().is_field_2d());
    }

    #[test]
    fn test_septagon() {
        assert!(BoundTester::<Septagon>::default().is_field_2d());
    }

    #[test]
    fn test_octagon() {
        assert!(BoundTester::<Octagon>::default().is_field_2d());
    }

    #[test]
    fn test_nonagon() {
        assert!(BoundTester::<Nonagon>::default().is_field_2d());
    }

    #[test]
    fn test_decagon() {
        assert!(BoundTester::<Decagon>::default().is_field_2d());
    }
}
