//! Create a 3D distance field by sweeping a 2D distance field
//! around the perimiter of another 2D distance field

use rust_gpu_bridge::glam::{Vec2, Vec3, Vec3Swizzles};
use type_fields::Field;

use crate::prelude::{Distance, FieldFunction, Normal, Uv};

use super::{FieldOperator, Operator};

/// Create a 3D distance field by sweeping a 2D distance field
/// around the perimiter of another 2D distance field
#[derive(Debug, Default, Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct SweepOp;

impl<Core, Shell> FieldOperator<(Core, Shell), Vec2, Distance> for SweepOp
where
    Core: FieldFunction<f32, Distance>,
    Shell: FieldFunction<f32, Distance>,
{
    fn operator(
        &self,
        attr: Distance,
        (core, shell): &(Core, Shell),
        p: Vec2,
    ) -> <Distance as crate::prelude::Attribute>::Type {
        let q = core.evaluate(attr, p.x);
        shell.evaluate(attr, q)
    }
}

impl<Core, Shell> FieldOperator<(Core, Shell), Vec3, Distance> for SweepOp
where
    Core: FieldFunction<Vec2, Distance>,
    Shell: FieldFunction<Vec2, Distance>,
{
    fn operator(
        &self,
        attr: Distance,
        (core, shell): &(Core, Shell),
        p: Vec3,
    ) -> <Distance as crate::prelude::Attribute>::Type {
        let q = Vec2::new(core.evaluate(attr, p.truncate()), p.z);
        shell.evaluate(attr, q)
    }
}

impl<Core, Shell> FieldOperator<(Core, Shell), Vec3, Normal<Vec3>> for SweepOp
where
    Core: FieldFunction<Vec2, Distance>,
    Shell: FieldFunction<Vec2, Normal<Vec2>>,
{
    fn operator(&self, _: Normal<Vec3>, (core, shell): &(Core, Shell), p: Vec3) -> Vec3 {
        let q = Vec2::new(core.evaluate(Distance, p.truncate()), p.z);
        let n = shell.evaluate(Normal::<Vec2>::default(), q);
        let w = p.xy().normalize() * n.x;
        Vec3::new(w.x, w.y, n.y).into()
    }
}

impl<Core, Shell> FieldOperator<(Core, Shell), Vec3, Uv> for SweepOp
where
    Core: FieldFunction<Vec2, Distance> + FieldFunction<Vec2, Uv>,
    Shell: FieldFunction<Vec2, Uv>,
{
    fn operator(&self, attr: Uv, (core, shell): &(Core, Shell), p: Vec3) -> Vec2 {
        let dist_core = core.evaluate(Distance, p.truncate());
        let uv_core = core.evaluate(attr, p.truncate());
        let q = Vec2::new(dist_core, p.z);
        let uv_shell = shell.evaluate(attr, q);
        Vec2::new(uv_core.x, uv_shell.x + uv_shell.y)
    }
}

/// Uniformly scale a distance field.
pub type Sweep<Core, Shell> = Operator<SweepOp, (Core, Shell)>;

impl<Core, Shell> Sweep<Core, Shell> {
    pub fn core(&mut self) -> &mut Core {
        &mut self.target().0
    }

    pub fn shell(&mut self) -> &mut Shell {
        &mut self.target().1
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use rust_gpu_bridge::glam::Vec3;

    use crate::{
        prelude::{BoundChecker, Circle, Point, Sweep},
        test_op_attrs_3d,
    };

    #[test]
    fn test_sweep() {
        assert!(BoundChecker::<Vec3, Sweep::<Circle, Circle>>::default().is_field());
    }

    test_op_attrs_3d!(Sweep::<Point, Point>);
}
