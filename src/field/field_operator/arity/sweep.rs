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

impl<Core, Shell> FieldOperator<(Core, Shell), Distance<Vec2>> for SweepOp
where
    Core: Field<Distance<f32>>,
    Shell: Field<Distance<f32>>,
{
    fn operator(
        &self,
        (core, shell): &(Core, Shell),
        p: Vec2,
    ) -> <Distance<Vec2> as crate::prelude::Attribute>::Output {
        let q = core.field(p.x);
        shell.field(q)
    }
}

impl<Core, Shell> FieldOperator<(Core, Shell), Distance<Vec3>> for SweepOp
where
    Core: Field<Distance<Vec2>>,
    Shell: Field<Distance<Vec2>>,
{
    fn operator(
        &self,
        (core, shell): &(Core, Shell),
        p: Vec3,
    ) -> <Distance<Vec3> as crate::prelude::Attribute>::Output {
        let q = Vec2::new(core.field(p.truncate()), p.z);
        shell.field(q)
    }
}

impl<Core, Shell> FieldOperator<(Core, Shell), Normal<Vec3>> for SweepOp
where
    Core: Field<Distance<Vec2>>,
    Shell: Field<Normal<Vec2>>,
{
    fn operator(&self, (core, shell): &(Core, Shell), p: Vec3) -> Vec3 {
        let q = Vec2::new(core.field(p.truncate()), p.z);
        let n = shell.field(q);
        let w = p.xy().normalize() * n.x;
        Vec3::new(w.x, w.y, n.y).into()
    }
}

impl<Core, Shell> FieldOperator<(Core, Shell), Uv<Vec3>> for SweepOp
where
    Core: Field<Distance<Vec2>> + Field<Uv<Vec2>>,
    Shell: Field<Uv<Vec2>>,
{
    fn operator(&self, (core, shell): &(Core, Shell), p: Vec3) -> Vec2 {
        let dist_core = Field::<Distance<Vec2>>::field(core, p.truncate());
        let uv_core = Field::<Uv<Vec2>>::field(core, p.truncate());
        let q = Vec2::new(dist_core, p.z);
        let uv_shell = shell.field(q);
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
    use crate::{
        prelude::{BoundTester, Circle, Point, Sweep},
        test_op_attrs_3d,
    };

    #[test]
    fn test_sweep() {
        assert!(BoundTester::<Sweep::<Circle, Circle>>::default().is_field_3d());
    }

    test_op_attrs_3d!(Sweep::<Point, Point>);
}
