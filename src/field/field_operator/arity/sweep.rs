//! Create a 3D distance field by sweeping a 2D distance field
//! around the perimiter of another 2D distance field

use rust_gpu_bridge::glam::{Vec2, Vec3, Vec3Swizzles};
use type_fields::Field;

use crate::prelude::{Distance, Field, FieldOperator, Normal, Operator, Uv};

/// Create a 3D distance field by sweeping a 2D distance field
/// around the perimiter of another 2D distance field
#[derive(Debug, Default, Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct SweepOp;

impl<Core, Shell> FieldOperator<(Core, Shell), Vec2, Distance> for SweepOp
where
    Core: Field<f32, Distance>,
    Shell: Field<f32, Distance>,
{
    fn operator(
        &self,
        attr: Distance,
        (core, shell): &(Core, Shell),
        p: Vec2,
    ) -> <Distance as crate::prelude::Attribute>::Type {
        let q = core.field(attr, p.x);
        shell.field(attr, q)
    }
}

impl<Core, Shell> FieldOperator<(Core, Shell), Vec3, Distance> for SweepOp
where
    Core: Field<Vec2, Distance>,
    Shell: Field<Vec2, Distance>,
{
    fn operator(
        &self,
        attr: Distance,
        (core, shell): &(Core, Shell),
        p: Vec3,
    ) -> <Distance as crate::prelude::Attribute>::Type {
        let q = Vec2::new(core.field(attr, p.truncate()), p.z);
        shell.field(attr, q)
    }
}

impl<Core, Shell> FieldOperator<(Core, Shell), Vec3, Normal<Vec3>> for SweepOp
where
    Core: Field<Vec2, Distance>,
    Shell: Field<Vec2, Normal<Vec2>>,
{
    fn operator(&self, _: Normal<Vec3>, (core, shell): &(Core, Shell), p: Vec3) -> Vec3 {
        let q = Vec2::new(core.field(Distance, p.truncate()), p.z);
        let n = shell.field(Normal::<Vec2>::default(), q);
        let w = p.xy().normalize() * n.x;
        Vec3::new(w.x, w.y, n.y).into()
    }
}

impl<Core, Shell> FieldOperator<(Core, Shell), Vec3, Uv> for SweepOp
where
    Core: Field<Vec2, Distance> + Field<Vec2, Uv>,
    Shell: Field<Vec2, Uv>,
{
    fn operator(&self, attr: Uv, (core, shell): &(Core, Shell), p: Vec3) -> Vec2 {
        let dist_core = core.field(Distance, p.truncate());
        let uv_core = core.field(attr, p.truncate());
        let q = Vec2::new(dist_core, p.z);
        let uv_shell = shell.field(attr, q);
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
        prelude::{BoundTester, Circle, Point, Sweep},
        test_op_attrs_3d,
    };

    #[test]
    fn test_sweep() {
        assert!(BoundTester::<Vec3, Sweep::<Circle, Circle>>::default().is_field());
    }

    test_op_attrs_3d!(Sweep::<Point, Point>);
}
