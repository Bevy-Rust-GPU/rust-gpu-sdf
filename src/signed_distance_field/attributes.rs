//! Attributes that can be computed by a distance field

pub mod distance {
    use core::{
        fmt::Display,
        ops::{Deref, DerefMut},
    };

    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
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

    impl Display for Distance {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }
}

pub mod position {
    use core::{
        fmt::Display,
        ops::{Deref, DerefMut},
    };

    use rust_gpu_bridge::prelude::Vec3;

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct Position(pub Vec3);

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

    use rust_gpu_bridge::prelude::Vec3;

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct Normal(pub Vec3);

    impl Deref for Normal {
        type Target = Vec3;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Normal {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl From<Vec3> for Normal {
        fn from(value: Vec3) -> Self {
            Normal(value)
        }
    }

    impl From<Normal> for Vec3 {
        fn from(value: Normal) -> Self {
            value.0
        }
    }

    impl Display for Normal {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }
}

pub mod uv {
    use core::{
        fmt::Display,
        ops::{Deref, DerefMut},
    };

    use rust_gpu_bridge::prelude::Vec2;

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct Uv(pub Vec2);

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

    use rust_gpu_bridge::prelude::Vec3;

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct Tangent(pub Vec3);

    impl Deref for Tangent {
        type Target = Vec3;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Tangent {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl From<Vec3> for Tangent {
        fn from(value: Vec3) -> Self {
            Tangent(value)
        }
    }

    impl From<Tangent> for Vec3 {
        fn from(value: Tangent) -> Self {
            value.0
        }
    }

    impl Display for Tangent {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }
}

pub mod color {
    use core::{
        fmt::Display,
        ops::{Deref, DerefMut},
    };

    use rust_gpu_bridge::prelude::Vec4;

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct Color(pub Vec4);

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

    impl Display for Color {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }
}

