#![no_std]

pub mod bound_tester;
pub use type_fields;
pub mod attribute;
pub mod context;
pub mod field;

pub mod field_type_machine {
    use core::marker::PhantomData;

    use rust_gpu_bridge::{
        glam::{Vec2, Vec2Swizzles, Vec3},
        Abs, Asin, Atan2, Length, Normalize,
    };
    use type_fields::{
        t_funk::{Copointed, Pointed, Tagged},
        type_machine::instruction::Instruction,
    };

    use crate::{impl_newtype, prelude::RaycastOutput};

    pub enum Distance {}
    pub enum Normal {}
    pub enum Position {}
    pub enum Uv {}

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EuclideanMetric<Dim>(PhantomData<Dim>);

    impl<Dim> Instruction for EuclideanMetric<Dim>
    where
        for<'a> Dim: Clone + Length + 'a,
    {
        type Input<'a> = Tagged<Position, Dim>
    where
        Self: 'a;

        type Output = Tagged<Distance, f32>;

        fn exec<'a>(self, input: Self::Input<'a>) -> Self::Output {
            let p = input.copoint();
            Tagged::<Distance, _>::point(p.length())
        }
    }

    pub struct EuclideanNormal<Dim>(PhantomData<Dim>);

    impl<Dim> Instruction for EuclideanNormal<Dim>
    where
        for<'a> Dim: Normalize + 'a,
    {
        type Input<'a> = Tagged<Position, Dim>
    where
        Self: 'a;

        type Output = Tagged<Normal, Dim>;

        fn exec<'a>(self, input: Self::Input<'a>) -> Self::Output {
            let p = input.copoint();
            Tagged::<Normal, _>::point(p.normalize())
        }
    }

    pub struct SphereUv;

    impl Instruction for SphereUv {
        type Input<'a> = Tagged<Position, Vec3>
        where
            Self: 'a;

        type Output = Tagged<Uv, Vec2>;

        fn exec<'a>(self, input: Self::Input<'a>) -> Self::Output {
            let p = input.copoint();
            Tagged::<Uv, _>::point(Vec2::new(
                (p.x.atan2(p.z) / core::f32::consts::TAU) + 0.5,
                (p.y.asin() / core::f32::consts::PI) + 0.5,
            ))
        }
    }

    #[derive(Default, Copy, Clone, PartialEq)]
    pub struct UvGradient {
        pub axis: Vec2,
        pub epsilon: f32,
    }

    pub struct GradientTetrahedron<T>
    where
        T: Instruction + 'static,
    {
        inner: T,
        axis: T::Input<'static>,
        epsilon: f32,
    }

    impl_newtype!(
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct Gradient<T>(T);
    );

    impl<T> Instruction for GradientTetrahedron<T>
    where
        for<'a> T: Clone + Instruction,
        for<'a> T::Input<'a>: Clone + Pointed<Pointed = Vec3> + Copointed<Copointed = Vec3>,
        for<'a> T::Output: Pointed<Pointed = Vec3> + Copointed<Copointed = Vec3>,
    {
        type Input<'a> = T::Input<'a>
    where
        Self: 'a;

        type Output = Gradient<T::Output>;

        fn exec<'a>(self, input: Self::Input<'a>) -> Self::Output {
            let input = input.copoint();

            let k = Vec2::new(1.0, -1.0);
            let out = k.xyy()
                * self
                    .inner
                    .clone()
                    .exec(T::Input::point(input + k.xyy() * self.epsilon))
                    .copoint()
                    .dot(self.axis.clone().copoint())
                + k.yyx()
                    * self
                        .inner
                        .clone()
                        .exec(T::Input::point(input + k.yyx() * self.epsilon))
                        .copoint()
                        .dot(self.axis.clone().copoint())
                + k.yxy()
                    * self
                        .inner
                        .clone()
                        .exec(T::Input::point(input + k.yxy() * self.epsilon))
                        .copoint()
                        .dot(self.axis.clone().copoint())
                + k.xxx()
                    * self
                        .inner
                        .clone()
                        .exec(T::Input::point(input + k.xxx() * self.epsilon))
                        .copoint()
                        .dot(self.axis.clone().copoint());

            Gradient(T::Output::point(out))
        }
    }

    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
    pub struct Isosurface {
        pub delta: f32,
    }

    impl Instruction for Isosurface {
        type Input<'a> = Tagged<Distance, f32>
    where
        Self: 'a;

        type Output = Tagged<Distance, f32>;

        fn exec<'a>(self, input: Self::Input<'a>) -> Self::Output {
            let d = input.copoint();
            Tagged::<Distance, _>::point(d - self.delta)
        }
    }

    pub enum RaymarchSteps {}
    pub enum RayPosition {}
    pub enum RayDirection {}
    pub enum RayStart {}
    pub enum RayEnd {}

    /// Sphere tracer that operates with respect to a precomputed Lipschitz bound.
    ///
    /// Costs 1 extra divide per step compared to [`SphereTraceNaive`],
    /// but results in overall faster intersections.
    ///
    /// Note: The precomputed lipschitz bound `k` must be correct in respect to the
    /// provided SDF for accurate behaviour; incorrect values will result in visual artifacting.
    #[derive(Copy, Clone, PartialEq)]
    #[repr(C)]
    pub struct SphereTraceLipschitz<const MAX_STEPS: u32, T> {
        pub epsilon: f32,
        pub frac_1_k: f32,
        pub _phantom: PhantomData<T>,
    }

    impl<const MAX_STEPS: u32, T> Default for SphereTraceLipschitz<MAX_STEPS, T>
    where
        T: Default,
    {
        fn default() -> Self {
            SphereTraceLipschitz {
                epsilon: 0.0001,
                frac_1_k: 1.0 / (Self::falloff_k(1.0, 3.0) * 3.0),
                _phantom: Default::default(),
            }
        }
    }

    impl<const MAX_STEPS: u32, T> SphereTraceLipschitz<MAX_STEPS, T> {
        // Computes the global lipschitz bound of the falloff function
        // e: energy
        // R: radius
        fn falloff_k(e: f32, r: f32) -> f32 {
            1.72 * e.abs() / r
        }
    }

    impl<const MAX_STEPS: u32, T> Instruction for SphereTraceLipschitz<MAX_STEPS, T>
    where
        for<'a> T: Clone
            + Instruction<Input<'a> = Tagged<Position, Vec3>, Output = Tagged<Distance, f32>>
            + 'a,
    {
        type Input<'a> = (Tagged<RayPosition, Vec3>, Tagged<RayDirection, Vec3>, Tagged<RayStart, f32>, Tagged<RayEnd, f32>, T)
        where
            Self: 'a;

        type Output = (Tagged<Distance, f32>, Tagged<RaymarchSteps, u32>);

        fn exec<'a>(self, (eye, dir, start, end, inner): Self::Input<'a>) -> Self::Output {
            let eye = eye.copoint();
            let dir = dir.copoint();
            let start = start.copoint();
            let end = end.copoint();

            let mut out = RaycastOutput::default();

            let mut t = start;
            for i in 0..MAX_STEPS {
                let pos = eye + dir * t;
                let dist = inner
                    .clone()
                    .exec(Tagged::<Position, _>::point(pos))
                    .copoint();

                out.march_step(t, dist);

                if dist < 0.0 {
                    out.march_hit(i);
                    break;
                }

                t += self.epsilon.max(dist.abs() * self.frac_1_k);

                if t > end {
                    out.march_miss(i);
                    break;
                }
            }

            (
                Tagged::<Distance, _>::point(out.closest_dist),
                Tagged::<RaymarchSteps, _>::point(out.steps),
            )
        }
    }
}

pub mod util {
    #[macro_export]
    macro_rules! impl_newtype {
        (#[derive($($derive:ident),*)] pub struct $ident:ident($ty:ident);) => {
            #[derive( $($derive),* )]
            pub struct $ident(pub $ty);

            crate::impl_morphisms_newtype!($ident($ty));
            crate::impl_arith_newtype!($ident($ty));
        };

        (#[derive($($derive:ident),*)] pub struct $ident:ident<$ty:ident>($ty2:ident);) => {
            #[derive( $($derive),* )]
            pub struct $ident<$ty>(pub $ty);

            crate::impl_morphisms_newtype!($ident<$ty>);
            crate::impl_arith_newtype!($ident<$ty>);
        };
    }

    #[macro_export]
    macro_rules! impl_morphisms_newtype {
        ($ident:ident($ty:ident)) => {
            impl core::ops::Deref for $ident {
                type Target = $ty;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl core::ops::DerefMut for $ident {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl From<$ty> for $ident {
                fn from(value: $ty) -> Self {
                    $ident(value)
                }
            }

            impl type_fields::t_funk::Pointed for $ident {
                type Pointed = $ty;

                fn point(unit: Self::Pointed) -> Self {
                    $ident(unit)
                }
            }

            impl type_fields::t_funk::Copointed for $ident {
                type Copointed = $ty;

                fn copoint(self) -> Self::Copointed {
                    self.0
                }
            }
        };
        ($ident:ident<$ty:ident>) => {
            impl<$ty> core::ops::Deref for $ident<$ty> {
                type Target = $ty;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl<$ty> core::ops::DerefMut for $ident<$ty> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl<$ty> From<$ty> for $ident<$ty> {
                fn from(value: $ty) -> Self {
                    $ident(value)
                }
            }

            impl<$ty> type_fields::t_funk::Pointed for $ident<$ty> {
                type Pointed = $ty;

                fn point(unit: Self::Pointed) -> Self {
                    $ident(unit)
                }
            }

            impl<$ty> type_fields::t_funk::Copointed for $ident<$ty> {
                type Copointed = $ty;

                fn copoint(self) -> Self::Copointed {
                    self.0
                }
            }

            impl<$ty, F, O> type_fields::t_funk::Fmap<F> for $ident<$ty>
            where
                F: Fn($ty) -> O,
            {
                type Fmap = $ident<F::Output>;

                fn fmap(self, f: F) -> Self::Fmap {
                    type_fields::t_funk::Pointed::point(f(type_fields::t_funk::Copointed::copoint(
                        self,
                    )))
                }
            }

            impl<$ty, _Ty, O> type_fields::t_funk::Apply<_Ty> for $ident<$ty>
            where
                $ty: Fn(_Ty) -> O,
            {
                type Apply = O;

                fn apply(self, a: _Ty) -> Self::Apply {
                    type_fields::t_funk::Copointed::copoint(self)(a)
                }
            }

            impl<$ty, F, O> type_fields::t_funk::Chain<F> for $ident<$ty>
            where
                F: Fn($ty) -> O,
            {
                type Chain = F::Output;

                fn chain(self, f: F) -> Self::Chain {
                    f(type_fields::t_funk::Copointed::copoint(self))
                }
            }
        };
    }

    #[macro_export]
    macro_rules! impl_arith_newtype {
        ($ident:ident ($ty:ident)) => {
            //crate::impl_arith_newtype_1!(Neg, neg, $ident($ty));
            crate::impl_arith_newtype_2!(Add, add, $ident($ty));
            crate::impl_arith_newtype_2!(Sub, sub, $ident($ty));
            crate::impl_arith_newtype_2!(Mul, mul, $ident($ty));
            crate::impl_arith_newtype_2!(Div, div, $ident($ty));
        };
        ($ident:ident <$ty:ident>) => {
            //crate::impl_arith_newtype_1!(Neg, neg, $ident<$ty>);
            crate::impl_arith_newtype_2!(Add, add, $ident<$ty>);
            crate::impl_arith_newtype_2!(Sub, sub, $ident<$ty>);
            crate::impl_arith_newtype_2!(Mul, mul, $ident<$ty>);
            crate::impl_arith_newtype_2!(Div, div, $ident<$ty>);
        };
    }

    #[macro_export]
    macro_rules! impl_arith_newtype_1 {
        ($op:ident, $fn:ident, $ident:ident ($ty:ident)) => {
            impl core::ops::$op for $ident {
                type Output = Self;

                fn $fn(self) -> Self::Output {
                    $ident(self.0.$fn())
                }
            }
        };
        ($op:ident, $fn:ident, $ident:ident <$ty:ident>) => {
            impl<$ty> core::ops::$op for $ident<$ty>
            where
                $ty: core::ops::$op<Output = $ty>,
            {
                type Output = Self;

                fn $fn(self) -> Self::Output {
                    $ident(self.0.$fn())
                }
            }
        };
    }

    #[macro_export]
    macro_rules! impl_arith_newtype_2 {
        ($op:ident, $fn:ident, $ident:ident ($ty:ident)) => {
            impl<RHS> core::ops::$op<RHS> for $ident
            where
                $ty: core::ops::$op<RHS, Output = $ty>,
            {
                type Output = Self;

                fn $fn(self, rhs: RHS) -> Self::Output {
                    Self(self.0.$fn(rhs))
                }
            }
        };
        ($op:ident, $fn:ident, $ident:ident <$ty:ident>) => {
            impl<$ty, RHS> core::ops::$op<RHS> for $ident<$ty>
            where
                $ty: core::ops::$op<RHS, Output = $ty>,
            {
                type Output = Self;

                fn $fn(self, rhs: RHS) -> Self::Output {
                    Self(self.0.$fn(rhs))
                }
            }
        };
    }
}

pub mod prelude;

use rust_gpu_bridge::glam::{Vec2, Vec3};

pub type D1 = f32;
pub type D2 = Vec2;
pub type D3 = Vec3;

/// Free-standing [`Default::default()`] invocation
pub fn default<T: Default>() -> T {
    Default::default()
}
