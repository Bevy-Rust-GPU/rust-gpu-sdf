//! Attributes that can be computed by a distance field

pub mod distance {
    use core::ops::{Deref, DerefMut};

    #[cfg(not(feature = "spirv-std"))]
    use core::fmt::Display;

    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
    #[repr(C)]
    pub struct Distance(pub f32);

    impl Deref for Distance {
        type Target = f32;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Distance {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl From<f32> for Distance {
        fn from(value: f32) -> Self {
            Distance(value)
        }
    }

    impl From<Distance> for f32 {
        fn from(value: Distance) -> Self {
            value.0
        }
    }

    #[cfg(not(feature = "spirv-std"))]
    impl Display for Distance {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }
}

pub mod position {
    use core::ops::{Deref, DerefMut};

    #[cfg(not(feature = "spirv-std"))]
    use core::fmt::{Debug, Display};

    use rust_gpu_bridge::prelude::Vec3;

    #[derive(Default, Copy, Clone, PartialEq)]
    #[repr(C)]
    pub struct Position(pub Vec3);

    #[cfg(not(feature = "spirv-std"))]
    impl Debug for Position {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl Deref for Position {
        type Target = Vec3;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Position {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl From<Vec3> for Position {
        fn from(value: Vec3) -> Self {
            Position(value)
        }
    }

    impl From<Position> for Vec3 {
        fn from(value: Position) -> Self {
            value.0
        }
    }

    #[cfg(not(feature = "spirv-std"))]
    impl Display for Position {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }
}

pub mod normal {
    use core::{
        fmt::Display,
        ops::{Deref, DerefMut},
    };

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    #[repr(C)]
    pub struct Normal<Dim>(pub Dim);

    impl<Dim> Deref for Normal<Dim> {
        type Target = Dim;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<Dim> DerefMut for Normal<Dim> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<Dim> From<Dim> for Normal<Dim> {
        fn from(value: Dim) -> Self {
            Normal(value)
        }
    }

    impl<Dim> Display for Normal<Dim>
    where
        Dim: Display,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }
}

pub mod uv {
    use core::ops::{Deref, DerefMut};

    #[cfg(not(feature = "spirv-std"))]
    use core::fmt::{Debug, Display};

    use rust_gpu_bridge::prelude::Vec2;

    #[derive(Default, Copy, Clone, PartialEq)]
    #[repr(C)]
    pub struct Uv(pub Vec2);

    #[cfg(not(feature = "spirv-std"))]
    impl Debug for Uv {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl Deref for Uv {
        type Target = Vec2;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Uv {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl From<Vec2> for Uv {
        fn from(value: Vec2) -> Self {
            Uv(value)
        }
    }

    impl From<Uv> for Vec2 {
        fn from(value: Uv) -> Self {
            value.0
        }
    }

    #[cfg(not(feature = "spirv-std"))]
    impl Display for Uv {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }
}

pub mod tangent {
    use core::{
        fmt::Display,
        ops::{Deref, DerefMut},
    };

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    #[repr(C)]
    pub struct Tangent<Dim>(pub Dim);

    impl<Dim> Deref for Tangent<Dim> {
        type Target = Dim;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<Dim> DerefMut for Tangent<Dim> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<Dim> From<Dim> for Tangent<Dim> {
        fn from(value: Dim) -> Self {
            Tangent(value)
        }
    }

    impl<Dim> Display for Tangent<Dim>
    where
        Dim: Display,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }
}

pub mod color {
    use core::ops::{Deref, DerefMut};

    #[cfg(not(feature = "spirv-std"))]
    use core::fmt::{Debug, Display};

    use rust_gpu_bridge::prelude::Vec4;

    #[derive(Default, Copy, Clone, PartialEq)]
    #[repr(C)]
    pub struct Color(pub Vec4);

    #[cfg(not(feature = "spirv-std"))]
    impl Debug for Color {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl Deref for Color {
        type Target = Vec4;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Color {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl From<Vec4> for Color {
        fn from(value: Vec4) -> Self {
            Color(value)
        }
    }

    impl From<Color> for Vec4 {
        fn from(value: Color) -> Self {
            value.0
        }
    }

    #[cfg(not(feature = "spirv-std"))]
    impl Display for Color {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }
}
